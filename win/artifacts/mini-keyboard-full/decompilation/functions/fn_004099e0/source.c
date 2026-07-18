
void __fastcall FUN_004099e0(int param_1)

{
  int iVar1;
  undefined4 *puVar2;
  char local_64 [16];
  char *local_54;
  undefined2 local_4d;
  undefined1 local_4b [3];
  undefined4 auStack_48 [14];
  undefined4 local_10;
  
  local_10 = 0;
  puVar2 = (undefined4 *)(local_4b + 3);
  for (iVar1 = 0xf; iVar1 != 0; iVar1 = iVar1 + -1) {
    *puVar2 = 0;
    puVar2 = puVar2 + 1;
  }
  _local_4b = 0xef;
  local_4d = 0xef03;
  if (DAT_0062046c == '\x01') {
    iVar1 = hid_write(*(undefined4 *)(param_1 + 0x968),&local_4d,0x41);
    local_64[0] = '\x02';
    local_64[1] = '\0';
    local_64[2] = '\0';
    local_64[3] = '\0';
    local_64[4] = '\0';
    local_64[5] = '\0';
    local_64[6] = '\0';
    local_64[7] = '\0';
    local_64[8] = '\0';
    local_64[9] = '\0';
    local_64[10] = '\0';
    local_64[0xb] = '\0';
    local_64[0xc] = '\0';
    local_64[0xd] = '\0';
    local_64[0xe] = '\0';
    local_64[0xf] = '\0';
    local_54 = "default";
    if (-1 < iVar1) {
      QMessageLogger::debug(local_64);
      return;
    }
    hid_error(*(undefined4 *)(param_1 + 0x968));
    QMessageLogger::debug(local_64);
  }
  return;
}

