#include <std.mi>
//#include "global.m"

Global Group mainGroup;
Global Button Play,Pause;
Global Layer Glow;

Global int alpha=0;

Global Float VolumeLevel;
Global Timer increase,decrease ;

System.onScriptLoaded() { 
	mainGroup = getScriptGroup();

	Play = mainGroup.findObject("Play");
	Pause = mainGroup.findObject("Pause");
	Glow = mainGroup.findObject("Glow");

	increase = new Timer;
	increase.setDelay(0);
	decrease = new Timer;
	decrease.setDelay(0);
}

System.onScriptUnloading() {
	delete increase; 
	delete decrease; 
}

Play.onEnterArea(){
	decrease.stop();
	increase.start();	
}
Play.onLeaveArea(){
	increase.stop();	
	decrease.start();	
}

Pause.onEnterArea(){
	decrease.stop();
	increase.start();	
}
Pause.onLeaveArea(){
	increase.stop();	
	decrease.start();	
}

increase.onTimer() {
	if(alpha<256)
	{
		Glow.setAlpha(alpha);
		alpha=alpha+51;
	}
//	increase.stop();
}

decrease.onTimer() {
	if(alpha>-1)
	{
		Glow.setAlpha(alpha);
		alpha=alpha-51;
	}
//	decrease.stop();
}