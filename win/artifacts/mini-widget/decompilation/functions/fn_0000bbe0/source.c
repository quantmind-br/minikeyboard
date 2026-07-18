
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
/* Widget::SetRgb_Led_Key(int) */

void Widget::SetRgb_Led_Key(int param_1)

{
  undefined1 uVar1;
  char cVar2;
  byte bVar3;
  uint uVar4;
  int in_ECX;
  int iVar5;
  undefined4 **ppuVar6;
  undefined4 local_70;
  undefined4 *local_6c;
  undefined *local_68;
  uint local_64;
  undefined4 local_60;
  code *local_50;
  int *local_40;
  int *local_3c;
  int *local_38;
  int *local_34;
  undefined4 local_30;
  undefined4 local_2c;
  undefined4 local_28;
  undefined4 local_24;
  undefined *local_20;
  
  ppuVar6 = &local_6c;
  local_30 = 2;
  local_2c = 0;
  local_28 = 0;
  local_24 = 0;
  local_40 = ___imp___ZN10QArrayData11shared_nullE;
  local_3c = ___imp___ZN10QArrayData11shared_nullE;
  local_38 = ___imp___ZN10QArrayData11shared_nullE;
  local_20 = &DAT_000022f1;
  iVar5 = (uint)DAT_00002435 * 3000;
  cVar2 = DAT_00002435 + 1;
  *(undefined1 *)(iVar5 + 0x100) = 0xfe;
  *(char *)(iVar5 + 0x102) = cVar2;
  uVar1 = DAT_000000de;
  *(undefined1 *)(iVar5 + 0x101) = 0xb0;
  *(undefined1 *)(iVar5 + 0x103) = uVar1;
  local_68 = &DAT_000025f2;
  local_64 = param_1;
  local_6c = &local_30;
  local_50 = ___imp___ZNK14QMessageLogger5debugEPKcz;
  local_70 = 0xbc71;
  (*___imp___ZNK14QMessageLogger5debugEPKcz)();
  local_64 = (uint)DAT_00002435;
  iVar5 = local_64 * 3000;
  if ((*(byte *)(in_ECX + 0x800 + (param_1 & 0xffU)) & 0xf0) == 0) {
    bVar3 = *(byte *)(iVar5 + 0x10b) & 0xf0;
    *(byte *)(iVar5 + 0x10b) = bVar3;
  }
  else {
    bVar3 = *(byte *)(iVar5 + 0x10b) & 0xf;
    *(byte *)(iVar5 + 0x10b) = bVar3;
  }
  bVar3 = bVar3 | *(byte *)(in_ECX + 0x800 + (param_1 & 0xffU));
  if ((bVar3 & 0xf0) == 0) {
    bVar3 = bVar3 | 0x50;
  }
  *(byte *)(local_64 * 3000 + 0x10b) = bVar3;
  uVar1 = DAT_000000de;
  uVar4 = (uint)DAT_00002434;
  local_30 = 2;
  local_2c = 0;
  local_28 = 0;
  local_24 = 0;
  local_20 = &DAT_000022f1;
  local_60 = 0;
  *(undefined1 *)(local_64 * 3000 + 0x109) = 1;
  local_68 = &DAT_000025b8;
  *(undefined1 *)(local_64 * 3000 + 0x103 + uVar4 * 0x32) = uVar1;
  *(undefined1 *)(local_64 * 0x3c + 0x40) = 1;
  local_6c = &local_30;
  local_70 = 0xbd31;
  (*local_50)();
  uVar4 = (uint)*(byte *)((uint)DAT_00002435 * 3000 + 0x10b);
  if (*(int *)(in_ECX + 0x2c) == 1) {
    if (DAT_00002436 == '\x02') {
      local_50 = ___imp___ZN7QStringaSERKS_;
    }
    else {
      local_6c = (undefined4 *)(in_ECX + 0x90c + ((int)uVar4 >> 4) * 4);
      local_50 = ___imp___ZN7QStringaSERKS_;
      local_70 = 0xbf6e;
      (*___imp___ZN7QStringaSERKS_)();
      ppuVar6 = (undefined4 **)&local_70;
      uVar4 = (uint)*(byte *)((uint)DAT_00002435 * 3000 + 0x10b);
    }
    iVar5 = in_ECX + 0x92c + (uVar4 & 0xf) * 4;
  }
  else {
    if (DAT_00002436 == '\x02') {
      local_50 = ___imp___ZN7QStringaSERKS_;
      ppuVar6 = &local_6c;
    }
    else {
      local_6c = (undefined4 *)(in_ECX + 0x8d8 + ((int)uVar4 >> 4) * 4);
      local_50 = ___imp___ZN7QStringaSERKS_;
      local_70 = 0xbe89;
      (*___imp___ZN7QStringaSERKS_)();
      ppuVar6 = (undefined4 **)&local_70;
      uVar4 = (uint)*(byte *)((uint)DAT_00002435 * 3000 + 0x10b);
    }
    iVar5 = in_ECX + 0x8f8 + (uVar4 & 0xf) * 4;
  }
  *ppuVar6 = (undefined4 *)iVar5;
  ppuVar6[-1] = (undefined4 *)0xbd82;
  (*local_50)();
  ppuVar6[1] = (undefined4 *)0x3;
  *ppuVar6 = (undefined4 *)&DAT_0000260f;
  ppuVar6[-1] = &local_34;
  ppuVar6[-2] = (undefined4 *)0xbda1;
  (*___imp___ZN7QString15fromUtf8_helperEPKci)();
  ppuVar6[-1] = &local_34;
  local_50 = ___imp___ZN7QString6appendERKS_;
  ppuVar6[-2] = (undefined4 *)0xbdb3;
  (*___imp___ZN7QString6appendERKS_)();
  if (*local_34 == 0) {
LAB_0000bea5:
    *ppuVar6 = (undefined4 *)0x4;
    ppuVar6[-1] = (undefined4 *)0x2;
    ppuVar6[-2] = local_34;
    ppuVar6[-3] = (undefined4 *)0xbebe;
    (*___imp___ZN10QArrayData10deallocateEPS_jj)();
  }
  else if (*local_34 != -1) {
    LOCK();
    *local_34 = *local_34 + -1;
    UNLOCK();
    if (*local_34 == 0) goto LAB_0000bea5;
  }
  ppuVar6[-2] = &local_40;
  ppuVar6[-3] = (undefined4 *)0xbddf;
  (*local_50)();
  ppuVar6[-3] = &local_3c;
  ppuVar6[-4] = (undefined4 *)0xbdf4;
  (*___imp___ZN9QTextEdit7setTextERK7QString)();
  if (*local_38 == 0) {
LAB_0000bec3:
    ppuVar6[-4] = local_38;
    ppuVar6[-2] = (undefined4 *)0x4;
    ppuVar6[-3] = (undefined4 *)0x2;
    ppuVar6[-5] = (undefined4 *)0xbedc;
    (*___imp___ZN10QArrayData10deallocateEPS_jj)();
    iVar5 = *local_3c;
    if (iVar5 != 0) goto LAB_0000be23;
LAB_0000bee9:
    ppuVar6[-4] = local_3c;
    ppuVar6[-2] = (undefined4 *)0x4;
    ppuVar6[-3] = (undefined4 *)0x2;
    ppuVar6[-5] = (undefined4 *)0xbf02;
    (*___imp___ZN10QArrayData10deallocateEPS_jj)();
    iVar5 = *local_40;
  }
  else {
    if (*local_38 != -1) {
      LOCK();
      *local_38 = *local_38 + -1;
      UNLOCK();
      if (*local_38 == 0) goto LAB_0000bec3;
    }
    iVar5 = *local_3c;
    if (iVar5 == 0) goto LAB_0000bee9;
LAB_0000be23:
    if (iVar5 != -1) {
      LOCK();
      *local_3c = *local_3c + -1;
      UNLOCK();
      if (*local_3c == 0) goto LAB_0000bee9;
    }
    iVar5 = *local_40;
  }
  if (iVar5 != 0) {
    if (iVar5 != -1) {
      LOCK();
      *local_40 = *local_40 + -1;
      UNLOCK();
      if (*local_40 == 0) goto LAB_0000bf0f;
    }
    return;
  }
LAB_0000bf0f:
  ppuVar6[-2] = (undefined4 *)0x4;
  ppuVar6[-3] = (undefined4 *)0x2;
  ppuVar6[-4] = local_40;
  ppuVar6[-5] = (undefined4 *)0xbf28;
  (*___imp___ZN10QArrayData10deallocateEPS_jj)();
  return;
}

