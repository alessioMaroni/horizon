package System is
   pragma Pure;

   type Name is (SYSTEM_NAME_UNKNOWN);
   System_Name : constant Name := SYSTEM_NAME_UNKNOWN;

   -- Target Architecture
   Word_Size        : constant := 32;
   Memory_Size      : constant := 2 ** 32;
   Address_Size     : constant := 32;
   Storage_Unit     : constant := 8;
   Word_Size_Bits   : constant := 32;

   -- Endianness
   Default_Bit_Order : constant Bit_Order := Low_Order_First;
   type Bit_Order is (High_Order_First, Low_Order_First);

   -- Alignment & Stack
   Max_Alignment    : constant := 4;
   Type_Invariant_Checks : constant Boolean := False;

   -- Types
   type Address is private;
   Null_Address : constant Address;

private
   type Address is mod 2**32;
   Null_Address : constant Address := 0;
end System;