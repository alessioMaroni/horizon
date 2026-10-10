package body Time is

   --  @brief Computes the absolute timestamp for the next timer expiration.
   --  @param Current_Mtime The current value of the machine time counter (mtime).
   --  @param Delta_Ticks   The interval duration expressed in clock ticks to add.
   --  @return              The resulting absolute target timestamp (`Mtime_Type`).
   --  @note                This function assumes that the addition does not overflow 
   --                       the range of Mtime_Type within the system's operational lifetime.
   function Compute_Next_Tick
     (Current_Mtime : Mtime_Type;
      Delta_Ticks   : Ticks_Type) return Mtime_Type is
   begin
      return Current_Mtime + Mtime_Type (Delta_Ticks);
   end Compute_Next_Tick; -- or Compute_Next_Tick

end Time;