
void FUN_0040da30(void)

{
  undefined1 uVar1;
  undefined1 extraout_AH;
  int iVar2;
  uint uVar3;
  uint uVar4;
  code *pcVar5;
  code *pcVar6;
  QArrayData *pQVar7;
  QArrayData *in_stack_ffffffcc;
  QArrayData *in_stack_ffffffd0;
  
  QString::fromAscii_helper(" ",1);
  iVar2 = QSpinBox::value();
  QString::number((int)&stack0xffffffd0,iVar2);
  QString::append((QString *)&stack0xffffffd0);
  if (*(int *)in_stack_ffffffd0 == 0) {
LAB_0040dc54:
    QArrayData::deallocate(in_stack_ffffffd0,2,4);
  }
  else if (*(int *)in_stack_ffffffd0 != -1) {
    LOCK();
    *(int *)in_stack_ffffffd0 = *(int *)in_stack_ffffffd0 + -1;
    UNLOCK();
    if (*(int *)in_stack_ffffffd0 == 0) goto LAB_0040dc54;
  }
  pcVar6 = (code *)&stack0xffffffcc;
  pcVar5 = (code *)0x40dace;
  QString::fromUtf8_helper((char *)pcVar6,0x4a1df0);
  pQVar7 = (QArrayData *)&stack0xffffffcc;
  (*pcVar5)();
  if (*(int *)in_stack_ffffffcc == 0) {
LAB_0040dc93:
    pQVar7 = (QArrayData *)0x2;
    pcVar6 = (code *)0x40dcac;
    QArrayData::deallocate(in_stack_ffffffcc,2,4);
  }
  else if (*(int *)in_stack_ffffffcc != -1) {
    LOCK();
    *(int *)in_stack_ffffffcc = *(int *)in_stack_ffffffcc + -1;
    UNLOCK();
    if (*(int *)in_stack_ffffffcc == 0) goto LAB_0040dc93;
  }
  QTextEdit::setText((QString *)&stack0xffffffc8);
  uVar1 = (*pcVar6)();
  (&DAT_0061e144)[(uint)DAT_00620475 * 3000 + (uint)DAT_00620474 * 0x32] = uVar1;
  (*pcVar6)();
  uVar3 = (uint)DAT_00620474;
  uVar4 = (uint)DAT_00620475;
  iVar2 = uVar4 * 3000 + uVar3 * 0x32;
  (&DAT_0061e140)[iVar2 + 5] = extraout_AH;
  (&DAT_0061e140)[iVar2 + 3] = DAT_0049e0fe;
  (&DAT_0061e080)[uVar4 * 0x3c + uVar3] = 1;
  pcVar6 = debug_exref;
  QMessageLogger::debug((char *)&stack0xffffffd0);
  iVar2 = 0;
  do {
    (*pcVar6)();
    iVar2 = iVar2 + 1;
  } while (iVar2 != 0x32);
  if (*(int *)pQVar7 != 0) {
    if (*(int *)pQVar7 != -1) {
      LOCK();
      *(int *)pQVar7 = *(int *)pQVar7 + -1;
      UNLOCK();
      if (*(int *)pQVar7 == 0) goto LAB_0040dc72;
    }
    return;
  }
LAB_0040dc72:
  QArrayData::deallocate(pQVar7,2,4);
  return;
}

