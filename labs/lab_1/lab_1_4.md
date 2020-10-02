# Implement a custom control class

参考网页：
<https://docs.microsoft.com/en-us/windows/apps/winui/winui3/xaml-templated-controls-cppwinrt-winui3>

***

本实验在[实验 1_2](lab_1_2.md)中所生成的 winuiTest 项目的基础上增加
一个按钮和一个标签类，其中标签类按照参考网页所生成。整个步骤参照参考网页，
需要注意如下地方： 
1. namespace 的改变，参考网页的 namespace 是 BgLabelControlApp，而我们这里
winuiTest 的 namespace 是 winuiTest

2. BgLabelControl 的构造函数中 DefailtStyleKey 中的字符串要做相应更改,
将 BgLabelControlApp 改为 winuiTest:

```cpp
BgLabelControl() { DefaultStyleKey(winrt::box_value(L"BgLabelControlApp.BgLabelControl")); }
```

3. 注意 Design the default style for BgLabelControl 中一节，如果
你从课程网站的仓库 winuiTest 中拷贝的话，你需要将文件夹 "Themes" 拷贝到你
的项目中，并注意将相应的 local 的定义做如下修改：

```xml
xmlns:local="using:winuiTest"
```

4. 如果你从课程仓库 winuiTest 直接克隆，需要按照 [实验 1_2](lab_1_2.md)
来重新生成并安装 certificate

