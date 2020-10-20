# steps to create a repository

本文目标：
1. 在 github 上创建名称为 se 的空仓库
2. 在 gitee 上创建名称为 se 的空仓库
3. 将 github 上的 se 仓库 clone 到本地
4. 将本地的 se 仓库更新到 gitee 上的 se 仓库
5. 将本地的 se 仓库更新到 github 上的 se 仓库

## 1 在 github 上创建名称为 se 的空仓库

## 2 在 gitee 上创建名称为 se 的空仓库

## 3 将 github 上的 se 仓库 clone 到本地

可以通过 VS 界面进行 clone，或者直接使用 git 命令

```git
git clone https://github.com/jichenghu/se.git
```

## 4 将本地的 se 仓库更新到 gitee 上的 se 仓库
## 5 将本地的 se 仓库更新到 github 上的 se 仓库

```git
git remote add origin_github https://github.com/jichenghu/se.git
git remote add origin_gitee https://gitee.com/jichenghu/se.git

git push -u origin_github master
git push -u origin_gitee master
```



