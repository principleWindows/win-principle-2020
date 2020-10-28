# steps to create a repository

本文目标：
1. 在 github 上创建名称为 se 的空仓库
2. 在 gitee 上创建名称为 se 的空仓库
3. 将 github 上的 se 仓库 clone 到本地
4. 将本地的 se 仓库更新到 gitee 上的 se 仓库
5. 将本地的 se 仓库更新到 github 上的 se 仓库

## 1 在 github 上创建名称为 se 的空仓库
1. 登录你的github，打开你的主页，在右上角点击"+"，选择"New repository"
![new rep](https://gitee.com/lpdink/win-principle-2020/raw/master/git/images/newRep.png)
2. 在新页面输入仓库名se，点击"Create repository"
![createRep](https://gitee.com/lpdink/win-principle-2020/raw/master/git/images/createRep.png)
3. 看到以下页面，空仓库创建完成。
![createRepFinish](https://gitee.com/lpdink/win-principle-2020/raw/master/git/images/createRepFinish.png)
## 2 在 gitee 上创建名称为 se 的空仓库
1. 登录你的gitee，打开你的主页，在右上角点击"+"，选择"新建仓库"
![gitee_newRep](https://gitee.com/lpdink/win-principle-2020/raw/master/git/images/gitee_newRep.png)
2. 在新页面输入仓库名se，点击"创建"
![gitee_createRep](https://gitee.com/lpdink/win-principle-2020/raw/master/git/images/gitee_createRep.png)
3. 看到以下页面，空仓库创建完成。
![gitee_finishRep](https://gitee.com/lpdink/win-principle-2020/raw/master/git/images/gitee_finishRep.png)
## 3 将 github 上的 se 仓库 clone 到本地

可以通过 VS 界面进行 clone，或者直接使用 git 命令,注意修改"jichenghu"为你的用户名。

你可以仓库首页点击code，选择clone下的https，复制来得到链接。在bash界面，通过shift+Insert完成粘贴。

```git
git clone https://github.com/jichenghu/se.git
```

## 4 将本地的 se 仓库更新到 gitee 上的 se 仓库
在完成本地git仓库的创建，并对项目文件进行必要的提交之后，你可以将仓库托管到gitee上。
注意修改"jichenghu"为你的用户名。

```git
git remote add origin_gitee https://gitee.com/jichenghu/se.git

git push -u origin_gitee master

```
## 5 将本地的 se 仓库更新到 github 上的 se 仓库
在完成本地git仓库的创建，并对项目文件进行必要的提交之后，你可以将仓库托管到github上。
注意修改"jichenghu"为你的用户名。

```git
git remote add origin_github https://github.com/jichenghu/se.git

git push -u origin_github master

```



