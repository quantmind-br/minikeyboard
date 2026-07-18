
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
/* Widget::InitPara() */

void Widget::InitPara(void)

{
  code *pcVar1;
  code *pcVar2;
  void *pvVar3;
  int in_ECX;
  undefined1 local_20 [16];
  
  Init_All_PHY_KEY_Value();
  pvVar3 = operator_new(8);
  (*___imp___ZN12QButtonGroupC1EP7QObject)();
  *(void **)(in_ECX + 0x958) = pvVar3;
  pcVar1 = ___imp___ZN12QButtonGroup9addButtonEP15QAbstractButtoni;
  (*___imp___ZN12QButtonGroup9addButtonEP15QAbstractButtoni)
            (*(undefined4 *)(*(int *)(in_ECX + 0x20) + 0x550),0);
  (*pcVar1)(*(undefined4 *)(*(int *)(in_ECX + 0x20) + 0x554),1);
  (*pcVar1)(*(undefined4 *)(*(int *)(in_ECX + 0x20) + 0x558),2);
  (*___imp___ZN15QAbstractButton10setCheckedEb)(1);
  pcVar1 = ___imp___ZN7QObject7connectEPKS_PKcS1_S3_N2Qt14ConnectionTypeE;
  (*___imp___ZN7QObject7connectEPKS_PKcS1_S3_N2Qt14ConnectionTypeE)
            (local_20,*(undefined4 *)(in_ECX + 0x958),&DAT_00002408);
  pcVar2 = ___imp___ZN11QMetaObject10ConnectionD1Ev;
  (*___imp___ZN11QMetaObject10ConnectionD1Ev)();
  (*pcVar1)(local_20,*(undefined4 *)(*(int *)(in_ECX + 0x20) + 0x4f0),&DAT_0000294b);
  (*pcVar2)();
  (*pcVar1)(local_20,*(undefined4 *)(*(int *)(in_ECX + 0x20) + 0x4f8),&DAT_0000294b);
  (*pcVar2)();
  (*pcVar1)(local_20,*(undefined4 *)(*(int *)(in_ECX + 0x20) + 0x4fc),&DAT_0000294b);
  (*pcVar2)();
  (*pcVar1)(local_20,*(undefined4 *)(*(int *)(in_ECX + 0x20) + 0x4f4),&DAT_0000294b);
  (*pcVar2)();
  (*pcVar1)(local_20,*(undefined4 *)(*(int *)(in_ECX + 0x20) + 0x5a0),&DAT_0000294b);
  (*pcVar2)();
  (*___imp___ZN7QWidget4hideEv)();
  InitBasic();
  GetAllBasicKey(0);
  return;
}

