# Rust/WinRT
***

## 1.引言

Rust/WinRT是继C++/WinRT之后，在Rust语言中构建的Windows运行时投影，可以令开发者在获得不次于C++的性能优势同时，避免内存泄露等问题。不仅如此，利用Rust的各种现代语言特性，以及便利的包管理工具，更能够进一步地提升开发效率。

若要开始使用Rust/WinRT进行开发，可能会需要这些资料：
[Github仓库](https://github.com/microsoft/winrt-rs)
[Docs.rs文档](https://docs.rs/winrt/)
[Crates.io](https://crates.io/crates/winrt)
除此之外，本文源代码中各个依赖项的使用方法，亦可以在Docs.rs中获得说明。

若需要Rust语言的基本知识以及环境配置等内容，请参见[Rust程序设计语言](http://120.78.128.153/rustbook/title-page.html)

在实际使用过程中，尽管__CLion__中提供了Rust语言插件，但经测试发现，它并不能很好地对winrt的生成文件进行代码提示，因此，还是推荐使用 __Visual Studio Code + rust-analyzer Extension__的组合。

### 1.1 依赖配置

由于Rust/WinRT今年(2020年)刚刚发布，在尚未达到1.0.0版本前各种接口/宏的使用方法都可能发生较大变化，因此请在`Cargo.toml`中指明`winrt`的版本号。
请将下列代码添加至`Cargo.toml`，它们会自动从Nuget中获取依赖文件：
```toml
[dependencies]
winrt = "0.7.2"

[package.metadata.winrt.dependencies]
"Microsoft.Windows.SDK.Contracts" = "10.0.19041.1"
```

尽管此时主要的依赖已经配置完成，实际上，可使用的winrt源文件还需要进一步生成。这是因为在实际开发过程中，为了尽可能提升编译速度，减少体积，winrt只会生成被使用的api文件。现在需要利用Rust的生成脚本(`build.rs`)文件来进行初次的生成工作。

根据官方指引，请将如下代码添加至`build.rs`，该文件默认应位于`Cargo.toml`的同级目录下，或者，也可以直接在`Cargo.toml`中通过配置指明生成脚本的位置。
```Rust
use winrt::*;

fn main() {
    build!(
        types
            windows::data::xml::dom::*
            windows::ui::*
    );
}
```

之后就可以在你的crate(crate的概念参见[Rust程序设计语言](http://120.78.128.153/rustbook/title-page.html))中使用上文代码中引入的组件了：
```Rust
use winrt::*;

mod bindings {
    include_bindings!();
}

fn main() -> Result<()> {
    use bindings::windows::data::xml::dom::*;

    let doc = XmlDocument::new()?;
    doc.load_xml("<html>hello world</html>")?;

    let root = doc.document_element()?;
    assert!(root.node_name()? == "html");
    assert!(root.inner_text()? == "hello world");

    Ok(())
}
```

之后在根crate目录下运行`cargo run`，cargo就会自动编译并运行上面的示例代码了。到此为止，文章已经向你基本展示了如何调用winrt API，下一节中会就“如何创建一个窗口程序”来提供一些线索。

### 1.2 创建winrt窗口程序

###### 1.2.1 编写必要的绑定代码

前文已经提到，Rust/WinRT仍处于开发阶段，因此诸如`Windows::foundation`等定义了C++/Windows中特定数据类型的API还不能直接使用，在winrt对其进行再次封装前，我们需要为程序中使用的特殊类型编写绑定代码。

让我们以`windows::foundation::TimeSpan`为例子，来展示如何编写绑定代码。在[WinRT API](https://docs.microsoft.com/en-us/uwp/api/?view=winrt-19041)中，这个namespace对应着`Windows.Foundation.TimeSpan`。正如上文中提到的那样，第一步需要在`build.rs`中引入这个类型，以使得对应的Rust源文件能够自动生成：
```Rust
winrt::build!(
    types
        windows::foundation::TimeSpan
);

fn main() {
    build();
}
```

由于每个`build.rs`文件都对应着一个crate，因此，上面的代码会在该`build.rs`文件对应的crate根目录中，生成一个名为`windows`的模块，而在这个模块中会递归生成在`build.rs`中引入的winrt API的模块，比如`foundation::TimeSpan`。这样，我们就可以通过`use crate::windows::foundation::TimeSpan;`来在新的`time_span.rs`文件中进一步编写绑定代码了。

之所以要编写`TimeSpan`的绑定，是因为在Rust中，原生的用于表示时间期间的数据类型为`std::time::Duration`，因此就需要能够将二者互相转换的代码。在Rust中，可以通过为该类型实现`std::convert`中的`From`/`TryInto`的trait来实现：
```Rust
impl From<Duration> for TimeSpan {
    fn from(duration: Duration) -> Self {
        TimeSpan {
            duration: (duration.as_millis() * 10000).try_into().unwrap(),
        }
    }
}
```

就按照这种方法，我们甚至可以为每一个winrt类型都编写一份绑定代码(一般直接用就可以了)，但可以预见的是，随着应用中的功能增加，很可能这类代码也会大量增加，若再令这些代码分散在主程序源码的根目录里就不够优雅了。一种推荐的做法是，在主程序crate的根目录下新建一个名为`bindings`的crate，这个新的crate应该作为所有绑定代码的根目录。理所当然的，为了主程序能够引用这些代码，入口文件应该为`lib.rs`(意为该crate为一个Rust库)。上文中曾经提到，winrt的Rust源文件会通过构建脚本在编译前生成，但除非特别设置，这些文件只会生成在`/target`目录(其实是通过`OUT_DIR`环境变量指定的目录)中，因此直接通过`mod`+`use`是无法进入其namespace的，需要在`lib.rs`中添加这行代码，以让编译器能够找到这些生成的源文件：
```Rust
include!(concat!(env!("OUT_DIR"), "/winrt.rs"));
```
大功告成！此时在主程序的`main.rs`中，可以通过`bindings`的路径来直接引用winrt API或编写的绑定代码了。如果IDE中未出现代码提示，可以尝试运行`cargo build`命令，使winrt生成Rust源文件。

###### 1.2.2 使用winrt + Rust Crate创建空白窗口程序

终于进入了本文的正题！首先介绍两个重要的Crate:
1. winit
   用于创建窗口/捕捉事件。当然，还有用于运行的事件循环创建。
2. raw-window-handle
   顾名思义，对窗口句柄的各种绑定以及操控API。

之后的工作是围绕着`Windows.UI.Composition`这个API来进行的。首先关注`window_target.rs`：
```Rust
pub trait CompositionDesktopWindowTargetSource {
    fn create_window_target(
        &self,
        compositor: &Compositor,
        is_topmost: bool,
    ) -> winrt::Result<DesktopWindowTarget>;
}
```
这里实现了一个如上定义的trait，简单来说，在具体实现中，对于一个具有`HasRawWindowHandle`trait的对象，我们可以通过该方法获取到其窗口句柄对象，以对其进行进一步设置。

还有一个`interop.rs`文件。再次强调，Rust/WinRT 仍处于开发阶段，因此一些工具/中间类型需要通过ABI进行手动绑定，这部分内容可以动过winrt API查阅，而具体的绑定方法，在搜索引擎中搜索`Rust FFI绑定`就能找到相关资料。长话短说，这部分主要是窗口线程控制相关的内容。

做好了准备工作，让我们回到`main.rs`，也就是主程序中来。主程序将会通过这个文件中的`run()`函数来实现，`main()`只要负责调用就好。

第一件事是进行初始化，也就是`interop`模块中的内容：
```Rust
    ro_initialize(RoInitType::MultiThreaded)?;
    let _controller = create_dispatcher_queue_controller_for_current_thread()?;
```
再通过`winit`crate来创建用于运行窗口的事件循环，并通过WindowBuilder创建窗口，此时就可以对标题进行设置了：
```Rust
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title("Rust/WinRT Sample Window");
```
WindowBuilder就是上文中提到的，实现了`HasRawWindowHandle`的对象，因此可以调用：
```Rust
    let compositor = Compositor::new()?;
    let target = window.create_window_target(&compositor, false)?;
    let root = compositor.create_container_visual()?;
    root.set_relative_size_adjustment(Vector2 { x: 1.0, y: 1.0 })?;
    target.set_root(&root)?;
```
此时我们获得了窗口`target`，并创建了compositor`root`为target调用`set_root()`。还可以对窗口进行其他设置，最终调用事件循环的`run()`函数，可在闭包的内部对各种事件利用`match`来进行模式匹配：
```Rust
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            
            _ => (),
        }
    });
```

完成！此时运行`cargo run`就可以看到空白窗口了。

###### 1.2.3 小结

现在我们应该对Rust/WinRT的使用有一定了解了。尽管暂时还没有WinUI之类的开发框架，至少可以不再使用相对古老的win32 API binding了。相信在Rust/WinRT进入Stable版本之后，会有更多基于它的Windows应用开发平台出现。
