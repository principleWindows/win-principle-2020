# The Windows Runtime (WinRT) type system

<https://docs.microsoft.com/en-us/uwp/winrt-cref/winrt-type-system>

***

## Runtime classes

WinRT allows you to define a class. A class must implement one or more 
interfaces. A class cannot implement type members directly (that is, it 
can't define its own methods, properties, nor events). A class must 
provide an implementation of all the members of all the interfaces it 
implements.

There are several different types of interfaces, which are described in 
detail below.
* Member interfaces (including protected and overridable interfaces)
* Static interfaces
* Activation factory interfaces
* Composition factory interfaces

Runtime classes cannot be parameterized. A runtime class may implement a 
parameterized interface instance (that is, a parameterized interface with 
all its type parameters specified) anywhere it would typically accept a 
non-parameterized interface.

A runtime class must have public visibility.

A runtime class may only implement interfaces that are not exclusive (that 
is, don¡¯t carry the exclusiveTo attribute) or that are exclusive to the 
runtime class in question. A runtime class may not implement interfaces 
that are exclusive to a different runtime class. There is one exception to 
this rule ¡ª a composable class may implement interfaces that are exclusive 
to a class in its derivation chain that is marked as overridable. Details 
on overridable interfaces to follow.




