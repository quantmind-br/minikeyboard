
void __thiscall FUN_0040d050(int param_1,byte param_2)

{
  undefined1 uVar1;
  int iVar2;
  uint uVar3;
  uint uVar4;
  char local_30 [16];
  char *local_20;
  char *local_1c;
  
  uVar3 = (uint)DAT_00620475;
  uVar4 = (uint)DAT_00620474;
  local_30[0] = '\x02';
  local_30[1] = '\0';
  local_30[2] = '\0';
  local_30[3] = '\0';
  local_30[4] = '\0';
  local_30[5] = '\0';
  local_30[6] = '\0';
  local_30[7] = '\0';
  local_30[8] = '\0';
  local_30[9] = '\0';
  local_30[10] = '\0';
  local_30[0xb] = '\0';
  local_30[0xc] = '\0';
  local_30[0xd] = '\0';
  local_30[0xe] = '\0';
  local_30[0xf] = '\0';
  local_20 = "default";
  iVar2 = uVar4 * 0x32 + uVar3 * 3000;
  (&DAT_0061e14b)[(uint)(byte)(&DAT_0061e149)[iVar2] * 2 + iVar2] =
       *(undefined1 *)(param_1 + 0x800 + (uint)param_2);
  uVar1 = DAT_0049e0fe;
  (&DAT_0061e149)[iVar2] = (&DAT_0061e149)[iVar2] + '\x01';
  (&DAT_0061e143)[iVar2] = uVar1;
  iVar2 = 0;
  (&DAT_0061e080)[uVar3 * 0x3c + uVar4] = 1;
  QMessageLogger::debug(local_30);
  do {
    local_30[4] = '\x02';
    local_30[5] = '\0';
    local_30[6] = '\0';
    local_30[7] = '\0';
    local_30[8] = '\0';
    local_30[9] = '\0';
    local_30[10] = '\0';
    local_30[0xb] = '\0';
    local_30[0xc] = '\0';
    local_30[0xd] = '\0';
    local_30[0xe] = '\0';
    local_30[0xf] = '\0';
    local_20 = (char *)0x0;
    local_1c = "default";
    iVar2 = iVar2 + 1;
    QMessageLogger::debug(local_30);
  } while (iVar2 != 0x32);
  return;
}

