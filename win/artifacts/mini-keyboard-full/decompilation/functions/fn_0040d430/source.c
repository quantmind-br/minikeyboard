
void __thiscall FUN_0040d430(int param_1,uint param_2)

{
  undefined1 uVar1;
  code *pcVar2;
  char cVar3;
  byte bVar4;
  uint uVar5;
  QArrayData *pQVar6;
  int iVar7;
  uint uVar8;
  code *pcVar9;
  QArrayData *pQVar10;
  QArrayData *local_34;
  undefined4 local_30;
  undefined4 local_2c;
  undefined4 local_28;
  undefined4 local_24;
  char *local_20;
  
  pcVar2 = shared_null_exref;
  local_30 = 2;
  local_2c = 0;
  local_28 = 0;
  local_24 = 0;
  local_20 = "default";
  iVar7 = (uint)DAT_00620475 * 3000;
  cVar3 = DAT_00620475 + 1;
  (&DAT_0061e140)[iVar7] = 0xfe;
  (&DAT_0061e142)[iVar7] = cVar3;
  uVar1 = DAT_0049e0fe;
  (&DAT_0061e141)[iVar7] = 0xb0;
  (&DAT_0061e143)[iVar7] = uVar1;
  pcVar9 = debug_exref;
  pQVar10 = (QArrayData *)pcVar2;
  pQVar6 = (QArrayData *)pcVar2;
  QMessageLogger::debug((char *)&local_30);
  uVar8 = (uint)DAT_00620475;
  iVar7 = uVar8 * 3000;
  if ((*(byte *)(param_1 + 0x800 + (param_2 & 0xff)) & 0xf0) == 0) {
    bVar4 = (&DAT_0061e14b)[iVar7] & 0xf0;
    (&DAT_0061e14b)[iVar7] = bVar4;
  }
  else {
    bVar4 = (&DAT_0061e14b)[iVar7] & 0xf;
    (&DAT_0061e14b)[iVar7] = bVar4;
  }
  bVar4 = bVar4 | *(byte *)(param_1 + 0x800 + (param_2 & 0xff));
  if ((bVar4 & 0xf0) == 0) {
    bVar4 = bVar4 | 0x50;
  }
  (&DAT_0061e14b)[uVar8 * 3000] = bVar4;
  uVar1 = DAT_0049e0fe;
  uVar5 = (uint)DAT_00620474;
  local_30 = 2;
  local_2c = 0;
  local_28 = 0;
  local_24 = 0;
  local_20 = "default";
  (&DAT_0061e149)[uVar8 * 3000] = 1;
  (&DAT_0061e143)[uVar5 * 0x32 + uVar8 * 3000] = uVar1;
  (&DAT_0061e080)[uVar8 * 0x3c] = 1;
  (*pcVar9)(&local_30,"Modify key value Flag: Layer=%d  Key=%d \n",uVar8,0);
  uVar8 = (uint)(byte)(&DAT_0061e14b)[(uint)DAT_00620475 * 3000];
  pcVar9 = operator=_exref;
  if (*(int *)(param_1 + 0x2c) == 1) {
    if (DAT_00620476 != '\x02') {
      QString::operator=((QString *)&stack0xffffffc0,
                         (QString *)(param_1 + 0x90c + ((int)uVar8 >> 4) * 4));
      uVar8 = (uint)(byte)(&DAT_0061e14b)[(uint)DAT_00620475 * 3000];
    }
    iVar7 = param_1 + 0x92c + (uVar8 & 0xf) * 4;
  }
  else {
    if (DAT_00620476 != '\x02') {
      QString::operator=((QString *)&stack0xffffffc0,
                         (QString *)(param_1 + 0x8d8 + ((int)uVar8 >> 4) * 4));
      uVar8 = (uint)(byte)(&DAT_0061e14b)[(uint)DAT_00620475 * 3000];
    }
    iVar7 = param_1 + 0x8f8 + (uVar8 & 0xf) * 4;
  }
  (*pcVar9)(iVar7);
  QString::fromUtf8_helper((char *)&local_34,0x4a1dbf);
  pcVar9 = append_exref;
  QString::append((QString *)&local_34);
  if (*(int *)local_34 == 0) {
LAB_0040d6f5:
    QArrayData::deallocate(local_34,2,4);
  }
  else if (*(int *)local_34 != -1) {
    LOCK();
    *(int *)local_34 = *(int *)local_34 + -1;
    UNLOCK();
    if (*(int *)local_34 == 0) goto LAB_0040d6f5;
  }
  (*pcVar9)(&stack0xffffffc0);
  QTextEdit::setText((QString *)&stack0xffffffc4);
  if (*(int *)pcVar2 == 0) {
LAB_0040d713:
    QArrayData::deallocate((QArrayData *)pcVar2,2,4);
    iVar7 = *(int *)pQVar6;
    if (iVar7 != 0) goto LAB_0040d673;
LAB_0040d739:
    QArrayData::deallocate(pQVar6,2,4);
    iVar7 = *(int *)pQVar10;
  }
  else {
    if (*(int *)pcVar2 != -1) {
      LOCK();
      *(int *)pcVar2 = *(int *)pcVar2 + -1;
      UNLOCK();
      if (*(int *)pcVar2 == 0) goto LAB_0040d713;
    }
    iVar7 = *(int *)pQVar6;
    if (iVar7 == 0) goto LAB_0040d739;
LAB_0040d673:
    if (iVar7 != -1) {
      LOCK();
      *(int *)pQVar6 = *(int *)pQVar6 + -1;
      UNLOCK();
      if (*(int *)pQVar6 == 0) goto LAB_0040d739;
    }
    iVar7 = *(int *)pQVar10;
  }
  if (iVar7 == 0) {
LAB_0040d75f:
    QArrayData::deallocate(pQVar10,2,4);
    return;
  }
  if (iVar7 != -1) {
    LOCK();
    *(int *)pQVar10 = *(int *)pQVar10 + -1;
    UNLOCK();
    if (*(int *)pQVar10 == 0) goto LAB_0040d75f;
  }
  return;
}

