© Copyright 2007 American Megatrends, Inc. All rights reserved.

AFUDOS v4.xx

AFUDOS is a BIOS update utility, also referred to as a "BIOS flash utility", with a command line interface for MS-DOS. The target board MUST use AMIBIOS Core 7.xx or AMIBIOS8. Older AMIBIOS systems support the AMIFLASH.COM program.

This utility offers the following features:
. Small executable file size
. Quickly update
. Clear updating information and status
. Fully compatible with previous version (See Appendix B AFUDOS v3.xx Commands)

Supported Operating System
. DOS environment (MS-DOS, FreeDOS, ...)

Getting Started:

Copy the AFUDOS.EXE executable file to any storage location accessible by the host system and then run AFUDOS in command prompt.

Basic Usage:

AFUDOS ROMFILE.ROM /P /C
. Program main BIOS image, reset CMOS to defaults

AFUDOS ROMFILE.ROM /P /B /N /C
. Program main BIOS image & bootblock image
. Reset CMOS and NVRAM to defaults


AFUWIN v4.xx

AFUWIN is an updating system BIOS utility with command line and GUI interface. It has same command line parameters and behavior as AFUDOS plus a simple graphical interface. The target board MUST use AMIBIOS Core 7.xx or AMIBIOS8. Older AMIBIOS systems support the AMIFLASH.COM program, which works in MS-DOS.

AFUWIN Utility is supported in following operating systems:
. Microsoft® Windows® 98
. Microsoft® Windows® ME
. Microsoft® Windows® 2000 (32bit & 64bit editions)
. Microsoft® Windows® NT 4.0
. Microsoft® Windows® XP/XP64
. Microsoft® Windows® PE
. Microsoft® Windows® Vista (32bit & 64bit editions)

Operating System Driver Requirements:
. UCOREVXD.VXD Driver for Microsoft® Windows® 98/ME
. UCORESYS.SYS Driver for Microsoft® Windows® NT/2000/XP/PE/Vista (32bit)
. UCOREW64.SYS Driver for Microsoft® Windows® XP64 & Vista (64bit)

Getting Started:

Copy AFUWIN.EXE, UCOREVXD.VXD, UCORESYS.SYS and UCOREW64.SYS to any storage location accessible by the host system and then run AFUWIN in command prompt. Remember that three files MUST be in same directory. For launching GUI mode, just double-click on AFUWIN.EXE. TO use in command line mode, run AFUWIN.EXE from a Microsoft® Windows® command prompt.

Usage & Example for command line mode (AFUDOS & AFUWIN).
Note: many of these options are advanced and can harm your system is used incorrectly.
 +---------------------------------------------------------------------------+
 |                   AMI Firmware Update Utility  Ver.4.xx                   |
 |      Copyright (C)2007 American Megatrends Inc. All Rights Reserved.      |
 +---------------------------------------------------------------------------+
 | Usage: AFUWIN <ROM File Name> [Option 1] [Option 2]...                    |
 |           or                                                              |
 |        AFUWIN <Input or Output File Name> <Command>                       |
 |           or                                                              |
 |        AFUWIN <Command>                                                   |
 | ------------------------------------------------------------------------- |
 | Commands:                                                                 |
 |         /O - Save current BIOS into file                                  |
 |         /U - Display ROM file's ROMID                                     |
 |        /Ln - Refer to Options: /Ln                                        |
 |         /M - Refer to Options: /M                                         |
 |       /MAI - Disaply System ROM and ROM file's MA information             |
 |     /HOLE: - Update specific ROM Hole according to given name.            |
 |              NewRomHole1.BIN /HOLE:RomH1                                  |
 |  /HOLEOUT: - Save specific ROM Hole according to given name.              |
 |              NewRomHole1.BIN /HOLE:RomH1                                  |
 |         /D - Verification test of given ROM File without flashing BIOS.   |
 |        /EC - Flash EC firmware BIOS (Refer to OFBD Spec)                  |
 |       /NCB - Flash NCB Area (Refer to OFBD Spec)                          |
 |    /NCBOUT - Output NCB Data according to given name.                     |
 | Options:                                                                  |
 |         /P - Program main BIOS image                                      |
 |         /B - Program Boot Block                                           |
 |         /N - Program NVRAM                                                |
 |         /C - Destroy CMOS checksum                                        |
 |         /E - Program Embedded Controller Block                            |
 |         /K - Program all non-critical blocks                              |
 |        /Kn - Program n'th non-critical block only(n=0-7)                  |
 |         /Q - Silent execution                                             |
 |    /REBOOT - Reboot after programming                                     |
 |         /X - Don't Check ROM ID                                           |
 |         /S - Display current system's ROMID                               |
 |        /Ln - Load CMOS defaults:(n=0-3)                                   |
 |              L0: Load current optimal                                     |
 |              L1: Load current failsafe                                    |
 |              L2: Load optimal from ROM file                               |
 |              L3: Load failsafe from ROM file                              |
 |         /M - Update bootblock MAC address if it exists                    |
 |              Example: /M1234ABCD will update MAC to 1234ABCD              |
 |         /R - Preserve ALL SMBIOS structure during NVRAM programming       |
 |      /ECUF - Update EC BIOS when newer version is detected.               |
 |  /SHUTDOWN - Shutdown after programming.                                  |
 +---------------------------------------------------------------------------+
