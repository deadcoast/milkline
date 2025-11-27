#include <std.mi>

Global Group avsGroup,avsBig,avscontent,avsButtonsMiddle,avsButtonsEdge,avsButtonsEdge2,IxIIx,IxIIxII;
Global string left;

System.onScriptLoaded() { 
	avscontent = getScriptGroup();
	avsGroup = avscontent.findObject("avsbottom");
	avsBig = avscontent.findObject("avsbig");
	avsButtonsMiddle = avscontent.findObject("avs.buttons.middle");
	avsButtonsEdge = avscontent.findObject("avs.buttons.edge");
	avsButtonsEdge2 = avscontent.findObject("avs.buttons.edge2");
	IxIIx= avscontent.findObject("avs.1x.2x");
	IxIIxII= avscontent.findObject("avs.1x.2x.2");
	avsButtonsEdge.hide();
}
avscontent.onResize(int x, int y, int w, int h) {
	left=integertostring(W/2-279);
	avsGroup.setXmlParam("x",left);
	avsButtonsMiddle.setXmlParam("x",left);
	avsButtonsEdge.setXmlParam("x",left);
	avsButtonsEdge2.setXmlParam("x",left);
	
	if (W<590) {
		avsGroup.hide();
		avsBig.hide();
		avsButtonsEdge.show();
		avsButtonsEdge2.hide();
		IxIIx.show();
		IxIIxII.hide();
	}
	else
	{
		avsGroup.show();
		avsBig.show();
		avsButtonsEdge.hide();
		avsButtonsEdge2.show();
		IxIIx.hide();
		IxIIxII.show();
	}
}

