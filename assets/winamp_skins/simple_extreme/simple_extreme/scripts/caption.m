#include <std.mi>

Global Group Caption;
Global Layer Logo;

System.onScriptLoaded() { 
	Caption = getScriptGroup();
	Logo= Caption.findObject("logo");
}

Caption.onResize(int x, int y, int w, int h) {
	Logo.setXmlParam("x",integertostring(W/2));
	
/*	if (W<590) {
//		avsGroup.hide();
	}
	else
	{
//		avsGroup.show();
	}*/
}

