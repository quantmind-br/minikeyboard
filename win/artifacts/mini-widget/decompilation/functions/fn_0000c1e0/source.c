
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
/* Widget::Key_Delay_Page_Opt() */

void Widget::Key_Delay_Page_Opt(void)

{
  code *pcVar1;
  code *pcVar2;
  undefined1 uVar3;
  undefined1 extraout_AH;
  undefined4 uVar4;
  uint uVar5;
  uint uVar6;
  int iVar7;
  int *local_38;
  int *local_34;
  int *local_30;
  undefined4 local_2c;
  undefined4 local_28;
  undefined4 local_24;
  undefined *local_20;
  
  local_38 = (int *)(*___imp___ZN7QString16fromAscii_helperEPKci)(&DAT_000025b4,1);
  pcVar2 = ___imp___ZNK8QSpinBox5valueEv;
  uVar4 = (*___imp___ZNK8QSpinBox5valueEv)();
  (*___imp___ZN7QString6numberEii)(&local_30,uVar4,10);
  pcVar1 = ___imp___ZN7QString6appendERKS_;
  (*___imp___ZN7QString6appendERKS_)(&local_30);
  if (*local_30 == 0) {
LAB_0000c404:
    (*___imp___ZN10QArrayData10deallocateEPS_jj)(local_30,2,4);
  }
  else if (*local_30 != -1) {
    LOCK();
    *local_30 = *local_30 + -1;
    UNLOCK();
    if (*local_30 == 0) goto LAB_0000c404;
  }
  (*___imp___ZN7QString15fromUtf8_helperEPKci)(&local_34,&DAT_00002640,3);
  (*pcVar1)(&local_34);
  if (*local_34 == 0) {
LAB_0000c443:
    (*___imp___ZN10QArrayData10deallocateEPS_jj)(local_34,2,4);
  }
  else if (*local_34 != -1) {
    LOCK();
    *local_34 = *local_34 + -1;
    UNLOCK();
    if (*local_34 == 0) goto LAB_0000c443;
  }
  (*___imp___ZN9QTextEdit7setTextERK7QString)(&local_38);
  uVar3 = (*pcVar2)();
  *(undefined1 *)((uint)DAT_00002434 * 0x32 + 0x104 + (uint)DAT_00002435 * 3000) = uVar3;
  (*pcVar2)();
  uVar5 = (uint)DAT_00002434;
  uVar6 = (uint)DAT_00002435;
  local_30 = (int *)0x2;
  local_2c = 0;
  local_28 = 0;
  local_24 = 0;
  local_20 = &DAT_000022f1;
  iVar7 = uVar5 * 0x32 + 0x100 + uVar6 * 3000;
  *(undefined1 *)(iVar7 + 5) = extraout_AH;
  *(undefined1 *)(iVar7 + 3) = DAT_000000de;
  *(undefined1 *)(uVar5 + 0x40 + uVar6 * 0x3c) = 1;
  pcVar1 = ___imp___ZNK14QMessageLogger5debugEPKcz;
  (*___imp___ZNK14QMessageLogger5debugEPKcz)(&local_30,&DAT_000025b8,uVar6,uVar5);
  iVar7 = 0;
  do {
    local_30 = (int *)0x2;
    local_2c = 0;
    local_28 = 0;
    local_24 = 0;
    local_20 = &DAT_000022f1;
    (*pcVar1)(&local_30,&DAT_000025e2,iVar7,
              *(undefined1 *)((uint)DAT_00002434 * 0x32 + 0x100 + (uint)DAT_00002435 * 3000 + iVar7)
             );
    iVar7 = iVar7 + 1;
  } while (iVar7 != 0x32);
  if (*local_38 != 0) {
    if (*local_38 != -1) {
      LOCK();
      *local_38 = *local_38 + -1;
      UNLOCK();
      if (*local_38 == 0) goto LAB_0000c422;
    }
    return;
  }
LAB_0000c422:
  (*___imp___ZN10QArrayData10deallocateEPS_jj)(local_38,2,4);
  return;
}

