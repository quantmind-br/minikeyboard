
void FUN_0040ff00(uint param_1)

{
  char local_20 [16];
  char *local_10;
  
  QTabWidget::setCurrentIndex(param_1);
  if (param_1 < 2) {
    DAT_0049e0fe = 1;
  }
  else if (param_1 == 2) {
    DAT_0049e0fe = 2;
  }
  else if (param_1 == 3) {
    DAT_0049e0fe = 8;
    DAT_00620474 = 0;
    FUN_0040fd00(0xff);
  }
  else if (param_1 == 4) {
    DAT_0049e0fe = 3;
  }
  else if (param_1 == 5) {
    DAT_0049e0fe = 5;
  }
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
  QMessageLogger::debug(local_20);
  return;
}

