# Debugging in Windows

[Symbols and Symbol Files](https://docs.microsoft.com/en-us/windows-hardware/drivers/debugger/symbols-and-symbol-files)
<br>
[Getting Started with Windows Debugging](https://docs.microsoft.com/en-us/windows-hardware/drivers/debugger/getting-started-with-windows-debugging)
<br>
[Debugging Tools for Windows (WinDbg, KD, CDB, NTSD)](https://docs.microsoft.com/en-us/windows-hardware/drivers/debugger/)
<br>
[Debugging Environments](https://docs.microsoft.com/en-us/windows-hardware/drivers/debugger/debuggers-in-the-debugging-tools-for-windows-package)
<br>
[Windows 用户态程序高效排错](userdbg_msinternal.pdf)
***

## 0 How to debug in Visual Studio

## 1 Standalone Debugging Tools for VS

There are six standalone debugging tools for Visual Studio:
* WinDbg Preview
* Windows Debugger (WinDbg)
* Kernel Debugger (KD)
* NTKD
* Console Debugger (CDB)
* NT Symbolic Debugger (NTSD)

### 1.1 WinDbg Preview
WinDbg Preview is the latest version of WinDbg with more modern visuals, 
faster windows, a full-fledged scripting experience, built with the 
extensible debugger data model front and center. WinDbg Preview is using 
the same underlying engine as WinDbg today, so all the commands, extensions, 
and workflows you're used to will still work as they did before.

For more information, see [Debugging Using WinDbg Preview](https://docs.microsoft.com/en-us/windows-hardware/drivers/debugger/debugging-using-windbg-preview)

### 1.2 WinDbg
Microsoft Windows Debugger (WinDbg) is a Windows-based debugger that is 
capable of both user-mode and kernel-mode debugging. WinDbg provides 
debugging for the Windows kernel, kernel-mode drivers, and system services, 
as well as user-mode applications and drivers.

WinDbg uses the Visual Studio debug symbol formats for source-level 
debugging. It can access any symbol or variable from a module that has PDB 
symbol files, and can access any public function's name that is exposed by 
modules that were compiled with COFF symbol files (such as Windows .dbg 
files).

WinDbg can view source code, set breakpoints, view variables (including C++ 
objects), stack traces, and memory. Its Debugger Command window allows the 
user to issue a wide variety of commands.

For kernel-mode debugging, WinDbg typically requires two computers (the host 
computer and the target computer). WinDbg also supports various remote 
debugging options for both user-mode and kernel-mode targets.

WinDbg is a graphical-interface counterpart to CDB/NTSD and to KD/NTKD.

## 1.3 KD
Microsoft Kernel Debugger (KD) is a character-based console program that 
enables in-depth analysis of kernel-mode activity on all NT-based operating 
systems. You can use KD to debug kernel-mode components and drivers, or to 
monitor the behavior of the operating system itself. KD also supports 
multiprocessor debugging.

Typically, KD does not run on the computer being debugged. You need two 
computers (the host computer and the target computer) for kernel-mode 
debugging.

[Debugging Using KD and NTKD](https://docs.microsoft.com/en-us/windows-hardware/drivers/debugger/debugging-using-kd-and-ntkd)

## 1.4 NTKD
There is a variation of the KD debugger named NTKD. It is identical to KD 
in every way, except that it spawns a new text window when it is started, 
whereas KD inherits the Command Prompt window from which it was invoked.

[Debugging Using KD and NTKD](https://docs.microsoft.com/en-us/windows-hardware/drivers/debugger/debugging-using-kd-and-ntkd)

## 1.5 CDB
Microsoft Console Debugger (CDB) is a character-based console program that 
enables low-level analysis of Windows user-mode memory and constructs. The 
name Console Debugger is used to indicate the fact that CDB is classified 
as a console application; it does not imply that the target application 
must be a console application. In fact, CDB is fully capable of debugging 
both console applications and graphical Windows programs.

CDB is extremely powerful for debugging a program that is currently running 
or has recently crashed (live analysis), yet simple to set up. It can be 
used to investigate the behavior of a working application. In the case of a 
failing application, CDB can be used to obtain a stack trace or to look at 
the guilty parameters. It works well across a network (using a remote access 
server), as it is character-based.

With CDB, you can display and execute program code, set breakpoints, and 
examine and change values in memory. CDB can analyze binary code by 
disassembling it and displaying assembly instructions. It can also analyze 
source code directly.

Because CDB can access memory locations through addresses or global symbols, 
you can refer to data and instructions by name rather than by address, 
making it easy to locate and debug specific sections of code. CDB supports 
debugging multiple threads and processes. It is extensible, and can read 
and write both paged and non-paged memory.

If the target application is itself a console application, the target will 
share the console window with CDB. To spawn a separate console window for a 
target console application, use the -2 command-line option.

## 1.6 NTSD
There is a variation of the CDB debugger named Microsoft NT Symbolic 
Debugger (NTSD). It is identical to CDB in every way, except that it spawns 
a new text window when it is started, whereas CDB inherits the Command 
Prompt window from which it was invoked.

Since the start command can also be used to spawn a new console window, 
the following two constructions will give the same results:

Console
```shell
start cdb parameters
ntsd parameters
```

It is possible to redirect the input and output from NTSD (or CDB) so that 
it can be controlled from a kernel debugger (either Visual Studio, WinDbg, 
or KD). If this technique is used with NTSD, no console window will appear 
at all. Controlling NTSD from the kernel debugger is therefore especially 
useful, since it results in an extremely light-weight debugger that places 
almost no burden on the computer containing the target application. This 
combination can be used to debug system processes, shutdown, and the later 
stages of boot up. See Controlling the User-Mode Debugger from the Kernel 
Debugger for details.