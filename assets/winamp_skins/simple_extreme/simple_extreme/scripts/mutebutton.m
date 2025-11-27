#include <std.mi>
//#include "global.m"

Global Group frameGroup;
Global Button MuteBtn,MuteBtnC;
Global Timer SongTickerTimer;
Global Float VolumeLevel;
Global Boolean Muted,BtnPressed;

Global Container main;
Global Layout normal;
Global Group volGroup;
Global Button MuteBtn2,MuteBtnC2;

System.onScriptLoaded() { 
	main=System.getContainer("main");
	
	normal= main.getLayout("normal");
	
	volGroup=normal.findObject("player.volume");
	
	MuteBtnC2 = volGroup.findObject("mutec");
	MuteBtn2 = volGroup.findObject("mute");
	
	Muted = getPrivateInt("winamp5", "muted", 0);
	VolumeLevel = getPrivateInt("winamp5", "old_volume", 0);
	frameGroup = getScriptGroup();
//
	MuteBtnC = frameGroup.findObject("mutec");
//
	MuteBtn = frameGroup.findObject("mute");
	MuteBtn.setActivated(Muted);

	if (Muted) {
		SongTickerTimer.start();
		MuteBtn.hide();
		MuteBtn2.hide();
	}
	else
	{
		MuteBtnC.hide();
		MuteBtnC2.hide();
	}
	
	VolumeLevel = System.getVolume();
	if (VolumeLevel<1) {
		MuteBtn.hide();
		MuteBtn2.hide();
		
		MuteBtnC.show();
		MuteBtnC2.show();
	}
	else
	{
		MuteBtnC.hide();
		MuteBtnC2.hide();
		
		MuteBtn.show();
		MuteBtn2.show();
	}
	BtnPressed = 0;
}

System.onScriptUnloading() {
	setPrivateInt("winamp5", "muted", Muted);
	setPrivateInt("winamp5", "old_volume", VolumeLevel);
}


MuteBtn.onLeftClick() {
	BtnPressed = 1;
		VolumeLevel = System.getVolume();
		System.setVolume(0);
		Muted = 1;
		
		MuteBtn.hide();
		MuteBtnC.show();
		
		MuteBtn2.hide();
		MuteBtnC2.show();
}

MuteBtnC.onLeftClick() {
	BtnPressed = 1;
		System.setVolume(VolumeLevel);
		Muted = 0;
		
		MuteBtnC.hide();
		MuteBtn.show();	

		MuteBtnC2.hide();
		MuteBtn2.show();		
}

System.onvolumechanged(int newvol)
{
	if (newvol<1) {
		MuteBtn.hide();
		MuteBtn2.hide();
		
		MuteBtnC.show();
		MuteBtnC2.show();
	}
	else
	{
		MuteBtnC.hide();
		MuteBtnC2.hide();
		
		MuteBtn.show();
		MuteBtn2.show();
	}
}
