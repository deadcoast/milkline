//-----------------------------------------------------------------------------
// play2pause.m
//
// Example of a Play/Pause Script
// created by Gonzotek & Vica.
//
// modified by FrisbeeMonkey
//-----------------------------------------------------------------------------

//                         USING THIS SCRIPT:
//*****************************************************************************
//  1.  Define play and pause buttons in your XML.
//  2.  Make sure their ids are "Play" and "Pause"
//  3.  Copy this script (and play2pause.maki) to your scripts folder.
//  4.  If you don't have play2pause.maki, compile this script.
//  5.  Add this line to the group that contains your play and pause buttons:
//        <script id="play2pause" file="scripts/play2pause.maki"/>
//  6.  Refresh your skin(F5) and try it out.
//*****************************************************************************

// never forget to include std.mi
#include </lib/std.mi>

//declares global variables for use in script
Global Group ButtonsGrp;
Global Button Play, Pause;
Global Layer PlayOverlay, PauseOverlay;
Global Boolean useOverlay;

//when the script is loaded, do this
System.onScriptLoaded() {

  //gets the group that has the objects we want
  ButtonsGrp = getScriptGroup();
  useOverlay = 1;
  //gets the "id" tags as defined in player-normal-group.xml for the two buttons
  Pause = ButtonsGrp.getObject("Pause");
  Play = ButtonsGrp.getObject("Play");
  PauseOverlay = ButtonsGrp.getObject("PauseButtonOverlay");
  if (PauseOverlay == NULL) useOverlay = 0;
  PlayOverlay = ButtonsGrp.getObject("PlayButtonOverlay");

  // hides both buttons until playing status is determined
  Pause.hide();
  Play.hide();
  if (useOverlay){
	  PauseOverlay.hide();
	  PlayOverlay.hide();
  }
  //determines whether Winamp is playing or paused, then shows the buttons accordingly
  if (System.getStatus()==1) {
    Pause.show();
    if (useOverlay) PauseOverlay.show();
  } else {
    Play.show();
    if (useOverlay) PlayOverlay.show();
  }
}//end onScriptLoaded()


// If winamp is playing, hides the play button and shows pause
System.onPlay()
{
  Play.hide();
  Pause.show();
  if (useOverlay) {
	  PlayOverlay.hide();
	  PauseOverlay.show();
  }
}//end onPlay()


// If winamp is paused, hides pause and shows play
System.onPause()
{
  Play.show();
  Pause.hide();
  if (useOverlay) {
	  PlayOverlay.show();
	  PauseOverlay.hide();
  }
}//end onPause()

// If winamp is stopped, shows play and hides pause
System.onStop()
{
  Play.show();
  Pause.hide();
  if (useOverlay) {
  	  PlayOverlay.show();
  	  PauseOverlay.hide();
  }
}//end onStop()


// After paused and button is again pressed starting play, will show pause and hide play
System.onResume()
{
  Play.hide();
  Pause.show();
  if (useOverlay) {
	  PlayOverlay.hide();
	  PauseOverlay.show();
  }
}