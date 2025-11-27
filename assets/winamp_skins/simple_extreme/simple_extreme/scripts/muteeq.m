#include <std.mi>
//#include "global.m"

Function updateVolume(int v);

Global Group frameGroup;
Global Button MuteBtn,MuteBtnC;
Global Timer SongTickerTimer;
Global Float VolumeLevel;
Global Boolean Muted,BtnPressed;
Global Layer volumebar;
Global Timer callback;

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

	callback = new Timer; callback.setDelay(5); callback.start();

	volumebar = frameGroup.findObject("volumebar");
	volumebar.setXmlParam("w",integertostring( (system.getVolume()/255) *100));

	SongTickerTimer = new Timer;
	SongTickerTimer.setDelay(1000);
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
	BtnPressed = 0;
}

System.onScriptUnloading() {
	setPrivateInt("winamp5", "muted", Muted);
	setPrivateInt("winamp5", "old_volume", VolumeLevel);
	delete callback;
}

SongTickerTimer.onTimer() {
	SongTickerTimer.stop();
}

MuteBtn.onLeftClick() {
	BtnPressed = 1;
		VolumeLevel = System.getVolume();
		System.setVolume(0);
		Muted = 1;
		SongTickerTimer.start();
		
		MuteBtn.hide();
		MuteBtnC.show();
		
		MuteBtn2.hide();
		MuteBtnC2.show();
}

MuteBtnC.onLeftClick() {
	BtnPressed = 1;
		System.setVolume(VolumeLevel);
		Muted = 0;
		SongTickerTimer.start();
		
		MuteBtnC.hide();
		MuteBtn.show();	

		MuteBtnC2.hide();
		MuteBtn2.show();		
}

System.onScriptUnloading() {
	delete SongTickerTimer;
}

System.onvolumechanged(int newvol)
{
///////////////////////	
	if (newvol<1) {
		MuteBtnC.show();
		MuteBtnC2.show();
		
		MuteBtn.hide();
		MuteBtn2.hide();
	}
	else
	{
		MuteBtn.show();
		MuteBtn2.show();
		
		MuteBtnC.hide();
		MuteBtnC2.hide();
	}
///////////////////////	

	volumebar.setXmlParam("w",integertostring( (newvol/255) *100));
	if (!BtnPressed) {
		SongTickerTimer.start();

		if (Muted) {
			MuteBtn.setActivated(0);

			Muted = 0;
		}
	}
	BtnPressed = 0;
}


