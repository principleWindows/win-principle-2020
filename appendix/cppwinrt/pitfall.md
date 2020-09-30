# Pitfalls and Bailout

***

|  | Issue | Recommendation |
| :---: | --- | --- |
| 1 | in the react-native-windows repo when a developer tries to reference a C# WinRT <br>component in their C++/WinRT app, they will propably hit a build error not being <br>able to find the C# WinRT XAML type info | add #include <winrt/Cam.Cam_XamlTypeInfo.h> in the cppwinrt PCH |
| 2 | error MIDL4006: [msg]A runtime class can derive only from a composable runtime class. | If you inherit from DependencyObject, you actually needn't implement INotifyPropertyChanged, and vice versa (even in WPF). Inheriting custom runtimeclass is not recommended in WinRT types, because JavaScript doesn't know inheritance, thus App Cert Kit will fail in that case. If you don't want to write the same logic each time, declare a C++ native class and implement the interfaces (DON'T inherit winrt::implement). Let the implementation classes inherit from it. <br><https://stackoverflow.com/questions/51572715/what-is-a-composable-runtime-class>|
| 3 | Microsoft .NET Framework Repair Tool | <https://www.microsoft.com/en-us/download/details.aspx?id=30135&751be11f-ede8-5a0c-058c-2ee190a24fa6=True> |
| 4 |  |  |


