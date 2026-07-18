
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
/* Widget::Set_SeleKeyValue(QString) */

void Widget::Set_SeleKeyValue(undefined4 param_1)

{
  char cVar1;
  code *pcVar2;
  int *piVar3;
  undefined4 uVar4;
  int iVar5;
  int **ppiVar6;
  int **ppiVar7;
  uint *puVar8;
  uint *puVar9;
  uint auStack_bc [14];
  int *local_84;
  int **local_80;
  int **local_7c;
  undefined4 local_78;
  code *local_58;
  code *local_54;
  code *local_50;
  int *local_40;
  int *local_3c;
  int *local_38;
  int *local_34;
  int *local_30;
  undefined4 local_2c;
  undefined4 local_28;
  undefined4 local_24;
  undefined *local_20;
  
  pcVar2 = ___imp___ZN7QString16fromAscii_helperEPKci;
  local_78 = 0;
  local_7c = (int **)&DAT_0000254c;
  local_80 = (int **)0xc6a2;
  local_40 = (int *)(*___imp___ZN7QString16fromAscii_helperEPKci)();
  local_78 = 0;
  local_7c = (int **)&DAT_0000254c;
  local_80 = (int **)0xc6b6;
  local_3c = (int *)(*pcVar2)();
  local_7c = &local_30;
  local_30 = (int *)0x2;
  local_2c = 0;
  local_28 = 0;
  local_24 = 0;
  local_20 = &DAT_000022f1;
  local_80 = (int **)0xc6f1;
  (*___imp___ZNK14QMessageLogger5debugEv)();
  local_78 = 0x1c;
  local_7c = (int **)&DAT_00002644;
  local_54 = ___imp___ZN7QString15fromUtf8_helperEPKci;
  local_84 = (int *)0xc717;
  local_80 = &local_34;
  (*___imp___ZN7QString15fromUtf8_helperEPKci)();
  local_84 = (int *)0xc722;
  local_80 = &local_34;
  (*___imp___ZN11QTextStreamlsERK7QString)();
  if (*local_34 == 0) {
LAB_0000c8c0:
    local_84 = local_34;
    local_7c = (int **)0x4;
    local_80 = (int **)0x2;
    auStack_bc[0xd] = 0xc8d9;
    (*___imp___ZN10QArrayData10deallocateEPS_jj)();
    cVar1 = (char)local_38[5];
  }
  else {
    if (*local_34 != -1) {
      LOCK();
      *local_34 = *local_34 + -1;
      UNLOCK();
      if (*local_34 == 0) goto LAB_0000c8c0;
    }
    cVar1 = (char)local_38[5];
  }
  ppiVar6 = &local_84;
  if (cVar1 != '\0') {
    local_84 = (int *)0x20;
    auStack_bc[0xd] = 0xc8f5;
    (*___imp___ZN11QTextStreamlsEc)();
    ppiVar6 = (int **)(auStack_bc + 0xd);
  }
  *ppiVar6 = (int *)(uint)DAT_00002434;
  ppiVar6[-1] = (int *)0xc763;
  (*___imp___ZN11QTextStreamlsEi)();
  ppiVar7 = ppiVar6 + -1;
  if ((char)local_38[5] != '\0') {
    ppiVar6[-1] = (int *)0x20;
    ppiVar6[-2] = (int *)0xca4d;
    (*___imp___ZN11QTextStreamlsEc)();
    ppiVar7 = ppiVar6 + -2;
  }
  *ppiVar7 = local_38;
  ppiVar7[-1] = (int *)0xc77e;
  (*___imp___ZN20QTextStreamFunctions4endlER11QTextStream)();
  ppiVar7[-1] = (int *)0xc78d;
  (*___imp___ZN6QDebugD1Ev)();
  *ppiVar7 = (int *)(uint)DAT_00002434;
  local_50 = ___imp___ZNK12QButtonGroup6buttonEi;
  ppiVar7[-1] = (int *)0xc7a4;
  uVar4 = (*___imp___ZNK12QButtonGroup6buttonEi)();
  ppiVar7[-1] = (int *)uVar4;
  local_58 = ___imp___ZNK15QAbstractButton4textEv;
  ppiVar7[-2] = (int *)0xc7b7;
  (*___imp___ZNK15QAbstractButton4textEv)();
  ppiVar7[2] = (int *)0x1;
  ppiVar7[1] = (int *)0xffffffff;
  *ppiVar7 = (int *)0x15;
  ppiVar7[-1] = (int *)local_30[1];
  ppiVar7[-2] = (int *)((int)local_30 + local_30[3]);
  ppiVar7[-3] = (int *)0xc7e8;
  iVar5 = (*___imp___ZN7QString14compare_helperEPK5QChariPKciN2Qt15CaseSensitivityE)();
  if (*local_30 == 0) {
LAB_0000c923:
    *ppiVar7 = (int *)0x4;
    ppiVar7[-1] = (int *)0x2;
    ppiVar7[-2] = local_30;
    ppiVar7[-3] = (int *)0xc93c;
    (*___imp___ZN10QArrayData10deallocateEPS_jj)();
  }
  else if (*local_30 != -1) {
    LOCK();
    *local_30 = *local_30 + -1;
    UNLOCK();
    if (*local_30 == 0) goto LAB_0000c923;
  }
  puVar8 = (uint *)(ppiVar7 + -2);
  if (iVar5 != 0) {
    ppiVar7[-2] = (int *)(uint)DAT_00002434;
    ppiVar7[-3] = (int *)0xc95c;
    uVar4 = (*local_50)();
    ppiVar7[-3] = (int *)uVar4;
    ppiVar7[-4] = (int *)0xc96a;
    (*local_58)();
    piVar3 = local_40;
    local_40 = local_38;
    local_38 = piVar3;
    if (*piVar3 == 0) {
LAB_0000c990:
      ppiVar7[-2] = (int *)0x4;
      ppiVar7[-3] = (int *)0x2;
      ppiVar7[-4] = piVar3;
      ppiVar7[-5] = (int *)0xc9ac;
      (*___imp___ZN10QArrayData10deallocateEPS_jj)();
    }
    else if (*piVar3 != -1) {
      LOCK();
      *piVar3 = *piVar3 + -1;
      UNLOCK();
      if (*piVar3 == 0) goto LAB_0000c990;
    }
    ppiVar7[-2] = (int *)0x1;
    ppiVar7[-3] = (int *)&DAT_000025b4;
    ppiVar7[-4] = (int *)&local_30;
    ppiVar7[-5] = (int *)0xc9cd;
    (*local_54)();
    ppiVar7[-4] = (int *)&local_30;
    ppiVar7[-5] = (int *)0xc9db;
    (*___imp___ZN7QString6appendERKS_)();
    if (*local_30 == 0) {
LAB_0000ca00:
      ppiVar7[-3] = (int *)0x4;
      ppiVar7[-4] = (int *)0x2;
      ppiVar7[-5] = local_30;
      ppiVar7[-6] = (int *)0xca19;
      (*___imp___ZN10QArrayData10deallocateEPS_jj)();
      puVar8 = (uint *)(ppiVar7 + -5);
    }
    else {
      puVar8 = (uint *)(ppiVar7 + -5);
      if (*local_30 != -1) {
        LOCK();
        *local_30 = *local_30 + -1;
        UNLOCK();
        puVar8 = (uint *)(ppiVar7 + -5);
        if (*local_30 == 0) goto LAB_0000ca00;
      }
    }
  }
  *puVar8 = param_1;
  puVar8[-1] = 0xc825;
  (*___imp___ZN7QStringaSERKS_)();
  puVar8[-1] = (uint)DAT_00002434;
  puVar8[-2] = 0xc83a;
  local_50 = (code *)(*local_50)();
  puVar8[-2] = (uint)&local_3c;
  puVar8[-3] = 0xc84b;
  uVar4 = (*___imp___ZN7QString6appendERKS_)();
  puVar8[-3] = uVar4;
  puVar8[-4] = 0xc85a;
  (*___imp___ZN15QAbstractButton7setTextERK7QString)();
  puVar9 = puVar8 + -4;
  if (DAT_00002434 != 0) {
    puVar8[-4] = (uint)&local_40;
    puVar8[-5] = 0xc878;
    (*___imp___ZN9QTextEdit7setTextERK7QString)();
    puVar9 = puVar8 + -5;
  }
  if (*local_3c == 0) {
LAB_0000ca20:
    puVar9[2] = 4;
    puVar9[1] = 2;
    *puVar9 = (uint)local_3c;
    puVar9[-1] = 0xca39;
    (*___imp___ZN10QArrayData10deallocateEPS_jj)();
  }
  else if (*local_3c != -1) {
    LOCK();
    *local_3c = *local_3c + -1;
    UNLOCK();
    if (*local_3c == 0) goto LAB_0000ca20;
  }
  if (*local_40 != 0) {
    if (*local_40 != -1) {
      LOCK();
      *local_40 = *local_40 + -1;
      UNLOCK();
      if (*local_40 == 0) goto LAB_0000c900;
    }
    return;
  }
LAB_0000c900:
  puVar9[2] = 4;
  puVar9[1] = 2;
  *puVar9 = (uint)local_40;
  puVar9[-1] = 0xc919;
  (*___imp___ZN10QArrayData10deallocateEPS_jj)();
  return;
}

