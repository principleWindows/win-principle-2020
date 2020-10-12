# Registry Structures and Registry Value Types

<https://docs.microsoft.com/en-us/windows/win32/api/winreg/ns-winreg-valenta>
<https://docs.microsoft.com/en-us/windows/win32/sysinfo/registry-value-types>
<https://docs.microsoft.com/en-us/windows/win32/sysinfo/registry-value-types>

***

## 1 Structures

Contains information about a registry value. The 
RegQueryMultipleValues function uses this structure.

```cpp
typedef struct value_entW {
  LPWSTR    ve_valuename;
  DWORD     ve_valuelen;
  DWORD_PTR ve_valueptr;
  DWORD     ve_type;
} VALENTW, *PVALENTW;
```

**ve_valuename**
<br>
The name of the value to be retrieved. Be sure to set this member 
before calling RegQueryMultipleValues.

**ve_valuelen**
<br>
The size of the data pointed to by ve_valueptr, in bytes.

**ve_valueptr**
<br>
A pointer to the data for the value entry. This is a pointer to 
the value's data returned in the lpValueBuf buffer filled in by 
RegQueryMultipleValues.

**ve_type**
<br>
The type of data pointed to by ve_valueptr. For a list of the 
possible types, see Registry Value Types.


## 2 Value Types

The following example walks a REG_MULTI_SZ string.
```cpp
#include <windows.h>
#include <tchar.h>
#include <stdio.h>

void SampleSzz(PTSTR pszz)
{
   _tprintf(_TEXT("\tBegin multi-sz string\n"));
   while (*pszz) 
   {
      _tprintf(_TEXT("\t\t%s\n"), pszz);
      pszz = pszz + _tcslen(pszz) + 1;
   }
   _tprintf(_TEXT("\tEnd multi-sz\n"));
}

int __cdecl main(int argc, char **argv)
{
   // Because the compiler adds a \0 at the end of quoted strings, 
   // there are two \0 terminators at the end. 

   _tprintf(_TEXT("Conventional multi-sz string:\n"));  
   SampleSzz(_TEXT("String1\0String2\0String3\0LastString\0"));

   _tprintf(_TEXT("\nTest case with no strings:\n"));  
   SampleSzz(_TEXT(""));

   return 0;
}
```


