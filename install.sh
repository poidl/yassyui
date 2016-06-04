LV2DIR=~/.lv2/
BACKUPDIR=~/programs/backup_lv2/
BUNDLE=yassyui.lv2
SONAME=libyassyui.so
DEST=$LV2DIR$BUNDLE
if [ -d "$DEST" ]; then
    if [ -d "$BACKUPDIR/$BUNDLE" ]; then
        rm -r $BACKUPDIR/$BUNDLE
    fi
    mv $DEST $BACKUPDIR/$BUNDLE
fi
#cp -r $BUNDLE $LV2DIR
# cp -r $BUNDLE/* $LV2DIR/yassy.lv2/
#cp target/debug/$SONAME $DEST
cp yassyui.lv2/yassyui.ttl ~/.lv2/yassy.lv2/
cp target/debug/$SONAME ~/.lv2/yassy.lv2/

# LV2DIR=~/.lv2/
# BACKUPDIR=~/programs/backup_lv2/
# BUNDLE=yassyui.lv2
# SONAME=libyassyui.so
# DEST=$LV2DIR$BUNDLE
# if [ -d "$DEST" ]; then
#     if [ -d "$BACKUPDIR/$BUNDLE" ]; then
#         rm -r $BACKUPDIR/$BUNDLE
#     fi
#     mv $DEST $BACKUPDIR/$BUNDLE
# fi
# cp -r $BUNDLE $LV2DIR
# cp -r $BUNDLE/* $LV2DIR/yassyui.lv2/
# cp target/debug/$SONAME $DEST
# # cp target/debug/$SONAME $DEST/../yassy.lv2/
