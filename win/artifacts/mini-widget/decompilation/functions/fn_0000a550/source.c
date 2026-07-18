
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
/* Widget::on_CleanButton_clicked() */

void Widget::on_CleanButton_clicked(void)

{
  byte bVar1;
  undefined1 *puVar2;
  uint uVar3;
  uint uVar4;
  int iVar5;
  int *local_20 [4];
  
  bVar1 = DAT_00002434;
  uVar4 = (uint)DAT_00002434;
  uVar3 = (uint)DAT_00002435;
  iVar5 = uVar4 * 0x32;
  puVar2 = (undefined1 *)(uVar3 * 3000 + 0x104 + iVar5);
  do {
    *puVar2 = 0;
    puVar2 = puVar2 + 1;
  } while ((undefined1 *)(uVar3 * 3000 + 0x132 + iVar5) != puVar2);
  if (bVar1 < 0x19) {
                    /* WARNING: Could not emulate address calculation at 0x0000a598 */
                    /* WARNING: Treating indirect jump as call */
    (**(code **)(&DAT_00002550 + uVar4 * 4))();
    return;
  }
  local_20[0] = (int *)(*___imp___ZN7QString16fromAscii_helperEPKci)(&DAT_0000254c,0);
  (*___imp___ZN9QTextEdit7setTextERK7QString)(local_20);
  if (*local_20[0] != 0) {
    if (*local_20[0] != -1) {
      LOCK();
      *local_20[0] = *local_20[0] + -1;
      UNLOCK();
      if (*local_20[0] == 0) goto LAB_0000a9e0;
    }
    return;
  }
LAB_0000a9e0:
  (*___imp___ZN10QArrayData10deallocateEPS_jj)(local_20[0],2,4);
  return;
}

