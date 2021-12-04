#include <efi.h>
#include <efilib.h>

#define INFO_LOG(STR) (Print(L"[INFO] %s\n", STR))

EFI_STATUS
EFIAPI
efi_main (EFI_HANDLE ImageHandle, EFI_SYSTEM_TABLE *SystemTable)
{
   InitializeLib(ImageHandle, SystemTable);
   Print(L"[INFO] Hello, Rusted-KalcOS\n");
   return EFI_SUCCESS;
}

