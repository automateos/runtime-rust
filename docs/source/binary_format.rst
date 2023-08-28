.. _binary_format:

========================================
AutomateOS Binary Format for executables
=========================================

.. contents:: Table of Contents
   :depth: 3

This document specifies the binary format of the executable files by the AutomateOS interpreter.

Magic Number
=============

Every executable file for the interpreter starts with the 4-byte hexadecimal number 0x61756F73, corresponding to "auos" in ASCII format.

Version
========

After the magic number, a 2-byte number corresponding to the version of the used binary format is introduced.

Timestamp
===========

After the version field, a 4-byte value follows with the timestamp encoded in little endian.

POU Type
==============

One byte value with the type of Program Organization Unit.

* **0x00** <POU> Program.
* **0x01** <FB> Function Block.
* **0x02** <FC> Function.
* **0x03** <GVL> Glovals Variables List.

POU identificador
=================

Sections
=========

Different sections of the file. Each section begins with a byte as an identifier, followed by 4 bytes indicating the size in bytes of the section's content, and then the content.

Code
-----
