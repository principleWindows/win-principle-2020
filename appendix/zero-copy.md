
# Zero Copy 

Windows supports zero copy through the win32 API function TransmitFile. 
Linux supports zero copy through system calls, such as sys/socket.h¡¯s sendfile, 
sendfile64, and splice.

<https://docs.microsoft.com/en-us/windows/win32/api/mswsock/nf-mswsock-transmitfile><br>
<https://docs.microsoft.com/en-us/windows-hardware/drivers/ddi/wdm/nf-wdm-zwwritefile><br>
<https://www.windowslatest.com/2020/08/25/microsoft-chromium-zero-copy-camera-capture/>






