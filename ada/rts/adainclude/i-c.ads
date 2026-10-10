package Interfaces.C is
   pragma Pure;

   -- Declarations for C intrinsic types
   type int        is new Integer;
   type short      is new Short_Integer;
   type long       is new Long_Integer;
   type unsigned   is mod 2 ** 32;
   type size_t     is mod 2 ** 32;

   type char       is new Character;
   type plain_char is new Character;

   type C_float    is new Float;
   type double     is new Long_Float;

   for int'Size use 32;
   for short'Size use 16;
   for long'Size use 32;
   for unsigned'Size use 32;
   for size_t'Size use 32;
   for char'Size use 8;
end Interfaces.C;