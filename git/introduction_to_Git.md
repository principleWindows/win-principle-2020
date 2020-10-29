# Introduction To Git 

This file is an introduction to Git, developing by some key points. Coding in the git bash environment. 

Some content quote from "ProGit"

* * *

## 什么是Git？为什么要使用它？

> Git is a free and open source distributed version control system designed to handle everything from small to very large projects with speed and efficiency.
> <p align="right">——https://git-scm.com/</p>

简单来说，Git是一套分布式的版本控制系统，它帮助我们记录一个或一组文件的某个历史状态，以便我们在任何时候将某个文件回溯到之前的状态，比较文件的细节变化，查找出谁在何时修改了哪个地方。

为什么我们需要使用Git呢？试想这样的情景：你致力于开发一个大型项目，已经完成了一些功能模块，但是现在需要继续拓展功能或修改功能，你不得不修改已完成的代码，但万一在修改过程中遇到麻烦，你可能因无法回到原先的版本而丢失已有的工作，你可以通过复制整个工程文件来做备份，但更省时省力的办法是使用Git。

此外，Git还允许我们将工程托管到云端，由于其分布式的特性，对多人联合开发至关重要。

## Git的文件状态
在开始使用Git之前，我们必须首先了解Git的文件状态。

Git有三种状态，你的文件可能处于其中之一：已提交（committed）、已修改（modified）和已暂存（staged）。

已提交：数据已经安全地保存在本地数据库中。

已修改：已经修改了文件，但还没保存到数据库中。

已暂存：对一个已修改文件的当前版本做了标记，使之包含在下一次提交的快照中。

于是引入Git项目的三个工作区域的概念：Git仓库、工作目录与暂存区域。

Git仓库：Git保存项目的元数据和对象数据库的地方。

工作目录：项目的某个版本独立从Git仓库抽取出的内容，供用户使用或修改。

暂存区域：保存了下次将提交的文件列表信息，一般在Git仓库目录中。


于是引申出Git的工作流程：

1. 在工作目录中修改文件。

2. 暂存文件，将文件的快照放入暂存区域。

3. 提交更新，找到暂存区域的文件，将快照永久性存储到Git仓库目录中。

## 开始使用Git
本篇以创建Git仓库，回溯项目版本为例，帮助你了解git的基本使用。我们将创建一个文件，修改它，之后用Git返回它修改之前的状态。你无需考虑集成开发环境，纯粹学习Git本身的基本概念和操作。

1. 通过[Git 下载地址 https://git-scm.com/downloads](https://git-scm.com/downloads)
下载对应你的操作系统的Git。

2. 安装完毕后，在任意目录新建文件夹，命名为gitTest，作为我们练习使用git的项目文件夹。

3. 打开gitTest，在空白位置右键鼠标，点击"Git Bash Here"。
![open_git_bash](https://gitee.com/lpdink/win-principle-2020/raw/master/git/images/open_git_bash.png)
4. 初始化Git仓库：在出现的命令行界面中键入"git init"，回车，这将创建一个.git的子目录，包含初始化的Git仓库的所有必须文件。但是git仍未对我们的项目文件进行追踪（实际上此时也没有文件:) ）
![git_init](https://gitee.com/lpdink/win-principle-2020/raw/master/git/images/git_init.png)
5. 新建文件：右键新建一个文本文件，命名为testFile.txt，用于模拟我们的项目文件。

6. 查看文件状态：在命令行界面输入命令"git status"，回车。由于我们新建了testFile.txt文件，你会看到"Untracked files: testFile.txt"的提示，告知我们文件"testFile.txt"尚未被追踪。
![git_status](https://gitee.com/lpdink/win-principle-2020/raw/master/git/images/git_status.png)
7. 暂存修改：输入命令"git add 文件名"（在本例中，文件名是testFile.txt)，回车。更常用的做法是"git add ." 这会将当前目录所有已修改未暂存的文件纳入暂存。

8. 提交修改：输入命令" git commit -m "你对本次提交的说明" "，回车。Git会找到暂存区域的所有文件，将这些文件的快照永久性地存储到Git仓库目录中。你可以修改“你对本次提交的说明”，来说明这次提交做了哪些修改。

9. 添加版本标签：输入命令"git tag v1.0"，回车。为这次提交的版本简单命名，这里命名为v1.0，实际上你可以任意命名。这不是必要的步骤，但在接下来我们回溯版本时很有用。
![add_commit_tag](https://gitee.com/lpdink/win-principle-2020/raw/master/git/images/add_commit_tag.png)
10. 修改文件：打开testFile.txt文件，任意修改其中的内容，比如输入数字123。保存文件。

11. 再次查看文件状态：切换回命令行界面，输入"git status"。Git告知我们<br>"Changes not staged for commit : <br>modified:  testFile.txt"
![modified](https://gitee.com/lpdink/win-principle-2020/raw/master/git/images/modified.png)
12. 再次暂存修改：输入"git add ."，回车。

13. 再次提交修改：输入"git commit -m "第二次提交的说明" "，回车。

14. 添加版本标签：输入"git tag v1.1"，回车。

15. 回溯版本：输入"git reset --hard v1.0" Git将当前目录回溯到你指定标签的版本（此处是v1.0)，打开testFile.txt，你会发现它重新成为一个空白文件了。如果要回溯回来，你应该再输入"git reset --hard v1.1"。
![reset](https://gitee.com/lpdink/win-principle-2020/raw/master/git/images/reset.png)


