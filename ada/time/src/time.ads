--  @brief Provides time-tracking primitives and scheduling tick calculations 
--         for the RISC-V timer subsystem, designed with SPARK contracts.

package Time
  with Pure
is
   --  @brief Represents the 64-bit machine time counter (mtime) value.
   type Mtime_Type is mod 2**64;
   
   --  @brief Represents a discrete duration measured in clock ticks.
   type Ticks_Type is range 1 .. 1_000_000_000;


  --  @brief Computes the absolute timestamp for the next timer expiration.
   --  @param Current_Mtime The current absolute value of the machine time counter.
   --  @param Delta_Ticks   The relative interval duration to add.
   --  @return              The calculated absolute target timestamp.
   --  @note                Verified via SPARK contracts (`Global` and `Post`).

   function Compute_Next_Tick
     (Current_Mtime : Mtime_Type;
      Delta_Ticks   : Ticks_Type) return Mtime_Type
   with
     Global        => null,
     Post          => Compute_Next_Tick'Result = Current_Mtime + Mtime_Type (Delta_Ticks),
     Export        => True,
     Convention    => C,
     External_Name => "spark_compute_next_tick";

end Time;