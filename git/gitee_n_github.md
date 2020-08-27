# 如何将本地 git 仓库同步推送到远程的 gitee 和 github 仓库



下面是个人摸索的一种简陋的方法, 纯手工操作风险大！

下述操作具有风险, 最好先备份 .git/ .git_hub/ .git_ee/ 三个文件夹

1. VS 安装 gitee 及 github 的 extension
2. 如果要同步到 github 则运行 git_hub.bat, 同时 disable gitee extension
并 enable github extension
3. 如果要同步到 gitee 则运行 gitee.bat, 同时 enable gitee extension
并 disable github extension
4. 使用 VS 提交 commit, 也可以直接在 powershell 中 git push
5. 注意在 .gitignore 中增加了 .git_hub/ 和 .git_ee/ 。如果你想仓库自动
备份 gitee 与 github 的不同提交以便出现操作风险后能手工回滚, 则可以将这
两个文件夹也储存到仓库中，但这样仓库的容量将会增长得非常迅猛。
6. backup.bat 是将当前的 .git/, .gib_hub/ 与 .git_ee/ 
如果存在的话就备份到 backups 文件夹中, 备份文件夹名称为当前日期和时间。
注意 .gitignore 中也加上了 backups/ , 该文件夹不会被推送到远程。
**随着时间的推移 backups 文件夹中的备份会越来越多，需要手工清除！**