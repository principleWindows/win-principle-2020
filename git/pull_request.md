# 处理 pull request 请求的相关命令

1. 建立临时分枝来保存该请求的修改，然后跳转到该临时分枝

```git
git checkout -b GuYith-master master
```

2. 拉起该分枝

```git
 git pull https://github.com/GuYith/win-principle-2020.git master
```

3. 使用工具检查修改


4. 切换到 master 分枝

```git
git checkout master
```

5. 合并分枝

```git
git merge GuYith-master
```

6. 推送

```git
git push origin_github master
```

7. 删除临时分枝

```git
git branch -D GuYith-master
```

8. 如果经常处理某个人的pull request, 可以给它设置一个别名

```git
git remote add origin_GuYith https://github.com/GuYith/win-principle-2020.git
```

这样在前面第2步拉起时可以使用别名减少输入

```git
 git pull origin_GuYith master
```



