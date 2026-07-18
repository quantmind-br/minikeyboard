
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
/* Widget::on_CleanALLButton_clicked() */

void Widget::on_CleanALLButton_clicked(void)

{
  code *pcVar1;
  code *pcVar2;
  undefined1 *puVar3;
  undefined1 *puVar4;
  uint uVar5;
  int *local_20 [4];
  
  uVar5 = (uint)DAT_00002435;
  puVar3 = (undefined1 *)(uVar5 * 3000 + 0x136);
  do {
    *puVar3 = 0;
    puVar3[0x32] = 0;
    puVar4 = puVar3 + 1;
    puVar3[100] = 0;
    puVar3[0x96] = 0;
    puVar3[200] = 0;
    puVar3[0xfa] = 0;
    puVar3[300] = 0;
    puVar3[0x15e] = 0;
    puVar3[400] = 0;
    puVar3[0x1c2] = 0;
    puVar3[500] = 0;
    puVar3[0x226] = 0;
    puVar3[600] = 0;
    puVar3[0x28a] = 0;
    puVar3[700] = 0;
    puVar3[0x2ee] = 0;
    puVar3[800] = 0;
    puVar3[0x352] = 0;
    puVar3[900] = 0;
    puVar3[0x3b6] = 0;
    puVar3[1000] = 0;
    puVar3[0x41a] = 0;
    puVar3[0x44c] = 0;
    puVar3[0x47e] = 0;
    pcVar1 = ___imp___ZN7QString16fromAscii_helperEPKci;
    puVar3 = puVar4;
  } while (puVar4 != (undefined1 *)(uVar5 * 3000 + 0x164));
  local_20[0] = (int *)(*___imp___ZN7QString16fromAscii_helperEPKci)(0x15,4);
  pcVar2 = ___imp___ZN15QAbstractButton7setTextERK7QString;
  (*___imp___ZN15QAbstractButton7setTextERK7QString)(local_20);
  if (*local_20[0] == 0) {
LAB_0000a180:
    (*___imp___ZN10QArrayData10deallocateEPS_jj)(local_20[0],2,4);
  }
  else if (*local_20[0] != -1) {
    LOCK();
    *local_20[0] = *local_20[0] + -1;
    UNLOCK();
    if (*local_20[0] == 0) goto LAB_0000a180;
  }
  local_20[0] = (int *)(*pcVar1)(0x15,4);
  (*pcVar2)(local_20);
  if (*local_20[0] == 0) {
LAB_0000a1a0:
    (*___imp___ZN10QArrayData10deallocateEPS_jj)(local_20[0],2,4);
  }
  else if (*local_20[0] != -1) {
    LOCK();
    *local_20[0] = *local_20[0] + -1;
    UNLOCK();
    if (*local_20[0] == 0) goto LAB_0000a1a0;
  }
  local_20[0] = (int *)(*pcVar1)(0x15,4);
  (*pcVar2)(local_20);
  if (*local_20[0] == 0) {
LAB_0000a1e0:
    (*___imp___ZN10QArrayData10deallocateEPS_jj)(local_20[0],2,4);
  }
  else if (*local_20[0] != -1) {
    LOCK();
    *local_20[0] = *local_20[0] + -1;
    UNLOCK();
    if (*local_20[0] == 0) goto LAB_0000a1e0;
  }
  local_20[0] = (int *)(*pcVar1)(0x15,4);
  (*pcVar2)(local_20);
  if (*local_20[0] == 0) {
LAB_0000a1c0:
    (*___imp___ZN10QArrayData10deallocateEPS_jj)(local_20[0],2,4);
  }
  else if (*local_20[0] != -1) {
    LOCK();
    *local_20[0] = *local_20[0] + -1;
    UNLOCK();
    if (*local_20[0] == 0) goto LAB_0000a1c0;
  }
  local_20[0] = (int *)(*pcVar1)(0x15,4);
  (*pcVar2)(local_20);
  if (*local_20[0] == 0) {
LAB_0000a3e0:
    (*___imp___ZN10QArrayData10deallocateEPS_jj)(local_20[0],2,4);
  }
  else if (*local_20[0] != -1) {
    LOCK();
    *local_20[0] = *local_20[0] + -1;
    UNLOCK();
    if (*local_20[0] == 0) goto LAB_0000a3e0;
  }
  local_20[0] = (int *)(*pcVar1)(0x15,4);
  (*pcVar2)(local_20);
  if (*local_20[0] == 0) {
LAB_0000a3c0:
    (*___imp___ZN10QArrayData10deallocateEPS_jj)(local_20[0],2,4);
  }
  else if (*local_20[0] != -1) {
    LOCK();
    *local_20[0] = *local_20[0] + -1;
    UNLOCK();
    if (*local_20[0] == 0) goto LAB_0000a3c0;
  }
  local_20[0] = (int *)(*pcVar1)(0x15,4);
  (*pcVar2)(local_20);
  if (*local_20[0] == 0) {
LAB_0000a3a0:
    (*___imp___ZN10QArrayData10deallocateEPS_jj)(local_20[0],2,4);
  }
  else if (*local_20[0] != -1) {
    LOCK();
    *local_20[0] = *local_20[0] + -1;
    UNLOCK();
    if (*local_20[0] == 0) goto LAB_0000a3a0;
  }
  local_20[0] = (int *)(*pcVar1)(0x15,4);
  (*pcVar2)(local_20);
  if (*local_20[0] == 0) {
LAB_0000a380:
    (*___imp___ZN10QArrayData10deallocateEPS_jj)(local_20[0],2,4);
  }
  else if (*local_20[0] != -1) {
    LOCK();
    *local_20[0] = *local_20[0] + -1;
    UNLOCK();
    if (*local_20[0] == 0) goto LAB_0000a380;
  }
  local_20[0] = (int *)(*pcVar1)(0x15,4);
  (*pcVar2)(local_20);
  if (*local_20[0] == 0) {
LAB_0000a4e0:
    (*___imp___ZN10QArrayData10deallocateEPS_jj)(local_20[0],2,4);
  }
  else if (*local_20[0] != -1) {
    LOCK();
    *local_20[0] = *local_20[0] + -1;
    UNLOCK();
    if (*local_20[0] == 0) goto LAB_0000a4e0;
  }
  local_20[0] = (int *)(*pcVar1)(0x15,4);
  (*pcVar2)(local_20);
  if (*local_20[0] == 0) {
LAB_0000a4c0:
    (*___imp___ZN10QArrayData10deallocateEPS_jj)(local_20[0],2,4);
  }
  else if (*local_20[0] != -1) {
    LOCK();
    *local_20[0] = *local_20[0] + -1;
    UNLOCK();
    if (*local_20[0] == 0) goto LAB_0000a4c0;
  }
  local_20[0] = (int *)(*pcVar1)(0x15,4);
  (*pcVar2)(local_20);
  if (*local_20[0] == 0) {
LAB_0000a4a0:
    (*___imp___ZN10QArrayData10deallocateEPS_jj)(local_20[0],2,4);
  }
  else if (*local_20[0] != -1) {
    LOCK();
    *local_20[0] = *local_20[0] + -1;
    UNLOCK();
    if (*local_20[0] == 0) goto LAB_0000a4a0;
  }
  local_20[0] = (int *)(*pcVar1)(0x15,4);
  (*pcVar2)(local_20);
  if (*local_20[0] == 0) {
LAB_0000a480:
    (*___imp___ZN10QArrayData10deallocateEPS_jj)(local_20[0],2,4);
  }
  else if (*local_20[0] != -1) {
    LOCK();
    *local_20[0] = *local_20[0] + -1;
    UNLOCK();
    if (*local_20[0] == 0) goto LAB_0000a480;
  }
  local_20[0] = (int *)(*pcVar1)(0x15,4);
  (*pcVar2)(local_20);
  if (*local_20[0] == 0) {
LAB_0000a460:
    (*___imp___ZN10QArrayData10deallocateEPS_jj)(local_20[0],2,4);
  }
  else if (*local_20[0] != -1) {
    LOCK();
    *local_20[0] = *local_20[0] + -1;
    UNLOCK();
    if (*local_20[0] == 0) goto LAB_0000a460;
  }
  local_20[0] = (int *)(*pcVar1)(0x15,4);
  (*pcVar2)(local_20);
  if (*local_20[0] == 0) {
LAB_0000a440:
    (*___imp___ZN10QArrayData10deallocateEPS_jj)(local_20[0],2,4);
  }
  else if (*local_20[0] != -1) {
    LOCK();
    *local_20[0] = *local_20[0] + -1;
    UNLOCK();
    if (*local_20[0] == 0) goto LAB_0000a440;
  }
  local_20[0] = (int *)(*pcVar1)(0x15,4);
  (*pcVar2)(local_20);
  if (*local_20[0] == 0) {
LAB_0000a420:
    (*___imp___ZN10QArrayData10deallocateEPS_jj)(local_20[0],2,4);
  }
  else if (*local_20[0] != -1) {
    LOCK();
    *local_20[0] = *local_20[0] + -1;
    UNLOCK();
    if (*local_20[0] == 0) goto LAB_0000a420;
  }
  local_20[0] = (int *)(*pcVar1)(0x15,4);
  (*pcVar2)(local_20);
  if (*local_20[0] == 0) {
LAB_0000a400:
    (*___imp___ZN10QArrayData10deallocateEPS_jj)(local_20[0],2,4);
  }
  else if (*local_20[0] != -1) {
    LOCK();
    *local_20[0] = *local_20[0] + -1;
    UNLOCK();
    if (*local_20[0] == 0) goto LAB_0000a400;
  }
  local_20[0] = (int *)(*pcVar1)(0x15,4);
  (*pcVar2)(local_20);
  if (*local_20[0] == 0) {
LAB_0000a2e0:
    (*___imp___ZN10QArrayData10deallocateEPS_jj)(local_20[0],2,4);
  }
  else if (*local_20[0] != -1) {
    LOCK();
    *local_20[0] = *local_20[0] + -1;
    UNLOCK();
    if (*local_20[0] == 0) goto LAB_0000a2e0;
  }
  local_20[0] = (int *)(*pcVar1)(0x15,4);
  (*pcVar2)(local_20);
  if (*local_20[0] == 0) {
LAB_0000a2c0:
    (*___imp___ZN10QArrayData10deallocateEPS_jj)(local_20[0],2,4);
  }
  else if (*local_20[0] != -1) {
    LOCK();
    *local_20[0] = *local_20[0] + -1;
    UNLOCK();
    if (*local_20[0] == 0) goto LAB_0000a2c0;
  }
  local_20[0] = (int *)(*pcVar1)(0x15,4);
  (*pcVar2)(local_20);
  if (*local_20[0] == 0) {
LAB_0000a2a0:
    (*___imp___ZN10QArrayData10deallocateEPS_jj)(local_20[0],2,4);
  }
  else if (*local_20[0] != -1) {
    LOCK();
    *local_20[0] = *local_20[0] + -1;
    UNLOCK();
    if (*local_20[0] == 0) goto LAB_0000a2a0;
  }
  local_20[0] = (int *)(*pcVar1)(0x15,4);
  (*pcVar2)(local_20);
  if (*local_20[0] == 0) {
LAB_0000a280:
    (*___imp___ZN10QArrayData10deallocateEPS_jj)(local_20[0],2,4);
  }
  else if (*local_20[0] != -1) {
    LOCK();
    *local_20[0] = *local_20[0] + -1;
    UNLOCK();
    if (*local_20[0] == 0) goto LAB_0000a280;
  }
  local_20[0] = (int *)(*pcVar1)(0x15,4);
  (*pcVar2)(local_20);
  if (*local_20[0] == 0) {
LAB_0000a260:
    (*___imp___ZN10QArrayData10deallocateEPS_jj)(local_20[0],2,4);
  }
  else if (*local_20[0] != -1) {
    LOCK();
    *local_20[0] = *local_20[0] + -1;
    UNLOCK();
    if (*local_20[0] == 0) goto LAB_0000a260;
  }
  local_20[0] = (int *)(*pcVar1)(0x15,4);
  (*pcVar2)(local_20);
  if (*local_20[0] == 0) {
LAB_0000a240:
    (*___imp___ZN10QArrayData10deallocateEPS_jj)(local_20[0],2,4);
  }
  else if (*local_20[0] != -1) {
    LOCK();
    *local_20[0] = *local_20[0] + -1;
    UNLOCK();
    if (*local_20[0] == 0) goto LAB_0000a240;
  }
  local_20[0] = (int *)(*pcVar1)(0x15,4);
  (*pcVar2)(local_20);
  if (*local_20[0] == 0) {
LAB_0000a220:
    (*___imp___ZN10QArrayData10deallocateEPS_jj)(local_20[0],2,4);
  }
  else if (*local_20[0] != -1) {
    LOCK();
    *local_20[0] = *local_20[0] + -1;
    UNLOCK();
    if (*local_20[0] == 0) goto LAB_0000a220;
  }
  local_20[0] = (int *)(*pcVar1)(0x15,4);
  (*pcVar2)(local_20);
  if (*local_20[0] == 0) {
LAB_0000a200:
    (*___imp___ZN10QArrayData10deallocateEPS_jj)(local_20[0],2,4);
  }
  else if (*local_20[0] != -1) {
    LOCK();
    *local_20[0] = *local_20[0] + -1;
    UNLOCK();
    if (*local_20[0] == 0) goto LAB_0000a200;
  }
  local_20[0] = (int *)(*pcVar1)(0x15,4);
  (*pcVar2)(local_20);
  if (*local_20[0] == 0) {
LAB_0000a360:
    (*___imp___ZN10QArrayData10deallocateEPS_jj)(local_20[0],2,4);
  }
  else if (*local_20[0] != -1) {
    LOCK();
    *local_20[0] = *local_20[0] + -1;
    UNLOCK();
    if (*local_20[0] == 0) goto LAB_0000a360;
  }
  local_20[0] = (int *)(*pcVar1)(0x15,4);
  (*pcVar2)(local_20);
  if (*local_20[0] == 0) {
LAB_0000a340:
    (*___imp___ZN10QArrayData10deallocateEPS_jj)(local_20[0],2,4);
  }
  else if (*local_20[0] != -1) {
    LOCK();
    *local_20[0] = *local_20[0] + -1;
    UNLOCK();
    if (*local_20[0] == 0) goto LAB_0000a340;
  }
  local_20[0] = (int *)(*pcVar1)(0x15,4);
  (*pcVar2)(local_20);
  if (*local_20[0] == 0) {
LAB_0000a321:
    (*___imp___ZN10QArrayData10deallocateEPS_jj)(local_20[0],2,4);
  }
  else if (*local_20[0] != -1) {
    LOCK();
    *local_20[0] = *local_20[0] + -1;
    UNLOCK();
    if (*local_20[0] == 0) goto LAB_0000a321;
  }
  local_20[0] = (int *)(*pcVar1)(&DAT_0000254c,0);
  (*___imp___ZN9QTextEdit7setTextERK7QString)(local_20);
  if (*local_20[0] != 0) {
    if (*local_20[0] != -1) {
      LOCK();
      *local_20[0] = *local_20[0] + -1;
      UNLOCK();
      if (*local_20[0] == 0) goto LAB_0000a300;
    }
    return;
  }
LAB_0000a300:
  (*___imp___ZN10QArrayData10deallocateEPS_jj)(local_20[0],2,4);
  return;
}

