
void __thiscall FUN_0040d810(int param_1,byte param_2)

{
  char cVar1;
  undefined1 uVar2;
  int iVar3;
  uint uVar4;
  uint uVar5;
  char local_20 [16];
  char *local_10;
  
  uVar4 = (uint)DAT_00620475;
  uVar5 = (uint)DAT_00620474;
  cVar1 = *(char *)(param_1 + 0x800 + (uint)param_2);
  if ((byte)(cVar1 + 0x9fU) < 0xb) {
    iVar3 = uVar5 * 0x32;
    switch(cVar1) {
    case 'a':
      (&DAT_0061e14b)[uVar4 * 3000 + iVar3] = 1;
      break;
    case 'b':
      (&DAT_0061e14b)[uVar4 * 3000 + iVar3] = 4;
      break;
    case 'c':
      (&DAT_0061e14b)[uVar4 * 3000 + iVar3] = 2;
      break;
    case 'd':
      (&DAT_0061e14e)[uVar4 * 3000 + iVar3] = 1;
      break;
    case 'e':
      (&DAT_0061e14e)[uVar4 * 3000 + iVar3] = 0xff;
      break;
    case 'f':
      iVar3 = uVar4 * 3000 + iVar3;
      (&DAT_0061e140)[iVar3 + 10] = 1;
      (&DAT_0061e140)[iVar3 + 0xe] = 1;
      break;
    case 'g':
      iVar3 = uVar4 * 3000 + iVar3;
      (&DAT_0061e140)[iVar3 + 10] = 1;
      (&DAT_0061e140)[iVar3 + 0xe] = 0xff;
      break;
    case 'h':
      iVar3 = uVar4 * 3000 + iVar3;
      (&DAT_0061e140)[iVar3 + 10] = 2;
      (&DAT_0061e140)[iVar3 + 0xe] = 1;
      break;
    case 'i':
      iVar3 = uVar4 * 3000 + iVar3;
      (&DAT_0061e140)[iVar3 + 10] = 2;
      (&DAT_0061e140)[iVar3 + 0xe] = 0xff;
      break;
    case 'j':
      iVar3 = uVar4 * 3000 + iVar3;
      (&DAT_0061e140)[iVar3 + 10] = 4;
      (&DAT_0061e140)[iVar3 + 0xe] = 1;
      break;
    case 'k':
      iVar3 = uVar4 * 3000 + iVar3;
      (&DAT_0061e140)[iVar3 + 10] = 4;
      (&DAT_0061e140)[iVar3 + 0xe] = 0xff;
    }
  }
  uVar2 = DAT_0049e0fe;
  local_20[0] = '\x02';
  local_20[1] = '\0';
  local_20[2] = '\0';
  local_20[3] = '\0';
  local_20[4] = '\0';
  local_20[5] = '\0';
  local_20[6] = '\0';
  local_20[7] = '\0';
  local_20[8] = '\0';
  local_20[9] = '\0';
  local_20[10] = '\0';
  local_20[0xb] = '\0';
  local_20[0xc] = '\0';
  local_20[0xd] = '\0';
  local_20[0xe] = '\0';
  local_20[0xf] = '\0';
  local_10 = "default";
  iVar3 = uVar4 * 3000 + uVar5 * 0x32;
  (&DAT_0061e140)[iVar3 + 9] = 1;
  (&DAT_0061e140)[iVar3 + 3] = uVar2;
  (&DAT_0061e080)[uVar4 * 0x3c + uVar5] = 1;
  iVar3 = 0;
  QMessageLogger::debug(local_20);
  do {
    local_20[4] = '\x02';
    local_20[5] = '\0';
    local_20[6] = '\0';
    local_20[7] = '\0';
    local_20[8] = '\0';
    local_20[9] = '\0';
    local_20[10] = '\0';
    local_20[0xb] = '\0';
    local_20[0xc] = '\0';
    local_20[0xd] = '\0';
    local_20[0xe] = '\0';
    local_20[0xf] = '\0';
    local_10 = (char *)0x0;
    iVar3 = iVar3 + 1;
    QMessageLogger::debug(local_20);
  } while (iVar3 != 0x32);
  return;
}

