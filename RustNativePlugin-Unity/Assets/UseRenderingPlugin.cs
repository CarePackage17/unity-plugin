using System;
using System.Collections;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using UnityEngine;

public class UseRenderingPlugin : MonoBehaviour
{
	//here's some sample code for reference:
	//https://github.com/Unity-Technologies/NativeRenderingPlugin/blob/master/UnityProject/Assets/UseRenderingPlugin.cs

	[DllImport("unity_plugin")]
	private static extern IntPtr DoGraphicsStuff();

    private IEnumerator Start()
    {
		yield return StartCoroutine(CallPluginAtEndOfFrames());
    }

    private IEnumerator CallPluginAtEndOfFrames()
	{
		while (true)
		{
			// Wait until all frame rendering is done
			yield return new WaitForEndOfFrame();

			// Issue a plugin event with arbitrary integer identifier.
			// The plugin can distinguish between different
			// things it needs to do based on this ID.
			// For our simple plugin, it does not matter which ID we pass here.
			GL.IssuePluginEvent(DoGraphicsStuff(), 1);
		}
	}
}
