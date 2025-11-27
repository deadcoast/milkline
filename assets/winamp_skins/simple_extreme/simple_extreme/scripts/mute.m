#include <std.mi>
//#include "global.m"

Function updateVolume(int v);

Global Group frameGroup;
Global Button MuteBtn,MuteBtnC;
Global Timer SongTickerTimer;
Global Text SongTicker;
Global Float VolumeLevel;
Global Boolean Muted,BtnPressed;
Global Layer volumebar;
Global Timer callback;

////////////////////////
/*
Global Container video;
Global Layout normal;
Global Group avsBar,avsEdge,volGroup;
Global Button MuteBtn2,MuteBtnC2;
*/
////////////////////////

System.onScriptLoaded() { 

////////////////////////
/*
	video=System.getContainer("Video");
	
	normal= video.getLayout("normal");
	
	avsBar=normal.findObject("avsbar");
	
	avsEdge=avsBar.findObject("avs.buttons.edge2");
	
	volGroup=avsEdge.findObject("AVS.volume");
	
	MuteBtnC2 = volGroup.findObject("mutec");
	MuteBtn2 = volGroup.findObject("mute");
	*/
////////////////////////
	
	Muted = getPrivateInt("winamp5", "muted", 0);
	VolumeLevel = getPrivateInt("winamp5", "old_volume", 0);
	frameGroup = getScriptGroup();
//
	MuteBtnC = frameGroup.findObject("mutec");
//
	MuteBtn = frameGroup.findObject("mute");
	MuteBtn.setActivated(Muted);

	callback = new Timer; callback.setDelay(5); callback.start();
	SongTicker = frameGroup.findObject("songticker");

	volumebar = frameGroup.findObject("volumebar");
	volumebar.setXmlParam("w",integertostring( (system.getVolume()/255) *100));

	SongTickerTimer = new Timer;
	SongTickerTimer.setDelay(1000);
	if (Muted) {
		SongTickerTimer.start();
		SongTicker.setText("Mute OFF");
		MuteBtn.hide();

////////////////////////
//		MuteBtn2.hide();
////////////////////////

	}
	else
	{
		MuteBtnC.hide();
////////////////////////
//		MuteBtnC2.hide();
////////////////////////
	}
	BtnPressed = 0;
}

System.onScriptUnloading() {
	setPrivateInt("winamp5", "muted", Muted);
	setPrivateInt("winamp5", "old_volume", VolumeLevel);
	delete callback;
}

SongTickerTimer.onTimer() {
	SongTicker.setText("");
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
		
////////////////////////
//		MuteBtn2.hide();
//		MuteBtnC2.show();
////////////////////////

		SongTicker.setText("Mute OFF");
}

MuteBtnC.onLeftClick() {
	BtnPressed = 1;
		System.setVolume(VolumeLevel);
		Muted = 0;
		SongTickerTimer.start();
		
		MuteBtnC.hide();
		MuteBtn.show();	

////////////////////////
//		MuteBtnC2.hide();
//		MuteBtn2.show();	
////////////////////////
	
		SongTicker.setText("Mute ON");
}

System.onScriptUnloading() {
	delete SongTickerTimer;
}

System.onvolumechanged(int newvol)
{
	volumebar.setXmlParam("w",integertostring( (newvol/255) *100));
	if (!BtnPressed) {
		SongTickerTimer.start();
		SongTicker.setText("Volume:" + System.integerToString(newvol/2.55) + "%");

		if (Muted) {
			MuteBtn.setActivated(0);

			Muted = 0;
		}
	}
	BtnPressed = 0;
}


