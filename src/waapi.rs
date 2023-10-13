use strum_macros::{Display, EnumString};

#[derive(Debug, EnumString, Display)]
pub enum Ak {
    // Retrieve global Wwise information.
    #[strum(serialize = "ak.wwise.core.getInfo")]
    WwiseCoreGetInfo,
    // Bring Wwise main window to foreground. Refer to SetForegroundWindow and AllowSetForegroundWindow on MSDN for more information on the restrictions. Refer to ak.wwise.core.getInfo to obtain the Wwise process ID for AllowSetForegroundWindow.
    #[strum(serialize = "ak.wwise.ui.bringToForeground")]
    WwiseUIBringToForeground,
    // Asynchronously post an Event to the sound engine (by event ID). See <tt>AK::SoundEngine::PostEvent</tt>.
    #[strum(serialize = "ak.soundengine.postEvent")]
    SoundEnginePostEvent,
    // Executes an action on all nodes that are referenced in the specified event in a Play action. See <tt>AK::SoundEngine::ExecuteActionOnEvent</tt>.
    #[strum(serialize = "ak.soundengine.executeActionOnEvent")]
    SoundEngineExecuteActionOnEvent,
    // Register a game object. Registering a game object twice does nothing. Unregistering it once unregisters it no matter how many times it has been registered. See <tt>AK::SoundEngine::RegisterGameObj</tt>.
    #[strum(serialize = "ak.soundengine.registerGameObj")]
    SoundEngineRegisterGameObj,
    // Stops the current content, associated to the specified playing ID, from playing. See <tt>AK::SoundEngine::StopPlayingID</tt>.
    #[strum(serialize = "ak.soundengine.stopPlayingID")]
    SoundEngineStopPlayingID,
    // Stop playing the current content associated to the specified game object ID. If no game object is specified, all sounds are stopped. See <tt>AK::SoundEngine::StopAll</tt>.
    #[strum(serialize = "ak.soundengine.stopAll")]
    SoundEngineStopAll,
    // Display a message in the Profiler's Capture Log view.
    #[strum(serialize = "ak.soundengine.postMsgMonitor")]
    SoundEnginePostMsgMonitor,
    // Set a game object's obstruction and occlusion levels. This function is used to affect how an object should be heard by a specific listener. See <tt>AK::SoundEngine::SetObjectObstructionAndOcclusion</tt>.
    #[strum(serialize = "ak.soundengine.setObjectObstructionAndOcclusion")]
    SoundEngineSetObjectObstructionAndOcclusion,
    // Set the output bus volume (direct) to be used for the specified game object. See <tt>AK::SoundEngine::SetGameObjectOutputBusVolume</tt>.
    #[strum(serialize = "ak.soundengine.setGameObjectOutputBusVolume")]
    SoundEngineSetGameObjectOutputBusVolume,
    // Sets the Auxiliary Busses to route the specified game object. See <tt>AK::SoundEngine::SetGameObjectAuxSendValues</tt>.
    #[strum(serialize = "ak.soundengine.setGameObjectAuxSendValues")]
    SoundEngineSetGameObjectAuxSendValues,
    // Posts the specified Trigger. See <tt>AK::SoundEngine::PostTrigger</tt>.
    #[strum(serialize = "ak.soundengine.postTrigger")]
    SoundEnginePostTrigger,
    // Sets the State of a Switch Group. See <tt>AK::SoundEngine::SetSwitch</tt>.
    #[strum(serialize = "ak.soundengine.setSwitch")]
    SoundEngineSetSwitch,
    // Sets the State of a State Group. See <tt>AK::SoundEngine::SetState</tt>.
    #[strum(serialize = "ak.soundengine.setState")]
    SoundEngineSetState,
    // Resets the value of a real-time parameter control to its default value, as specified in the Wwise project. See <tt>AK::SoundEngine::ResetRTPCValue</tt>.
    #[strum(serialize = "ak.soundengine.resetRTPCValue")]
    SoundEngineResetRTPCValue,
    // Sets the value of a real-time parameter control. See <tt>AK::SoundEngine::SetRTPCValue</tt>.
    #[strum(serialize = "ak.soundengine.setRTPCValue")]
    SoundEngineSetRTPCValue,
    // Sets a listener's spatialization parameters. This lets you define listener-specific volume offsets for each audio channel. See <tt>AK::SoundEngine::SetListenerSpatialization</tt>.
    #[strum(serialize = "ak.soundengine.setListenerSpatialization")]
    SoundEngineSetListenerSpatialization,
    // Sets multiple positions for a single game object. Setting multiple positions for a single game object is a way to simulate multiple emission sources while using the resources of only one voice. This can be used to simulate wall openings, area sounds, or multiple objects emitting the same sound in the same area. See <tt>AK::SoundEngine::SetMultiplePositions</tt>.
    #[strum(serialize = "ak.soundengine.setMultiplePositions")]
    SoundEngineSetMultiplePositions,
    // Sets the position of a game object. See <tt>AK::SoundEngine::SetPosition</tt>.
    #[strum(serialize = "ak.soundengine.setPosition")]
    SoundEngineSetPosition,
    // Sets the scaling factor of a game object. You can modify the attenuation computations on this game object to simulate sounds with a larger or smaller affected areas. See <tt>AK::SoundEngine::SetScalingFactor</tt>.
    #[strum(serialize = "ak.soundengine.setScalingFactor")]
    SoundEngineSetScalingFactor,
    // Sets the default active listeners for all subsequent game objects that are registered. See <tt>AK::SoundEngine::SetDefaultListeners</tt>.
    #[strum(serialize = "ak.soundengine.setDefaultListeners")]
    SoundEngineSetDefaultListeners,
    // Sets a single game object's active listeners. By default, all new game objects have no listeners active, but this behavior can be overridden with <tt>SetDefaultListeners()</tt>. Inactive listeners are not computed. See <tt>AK::SoundEngine::SetListeners</tt>.
    #[strum(serialize = "ak.soundengine.setListeners")]
    SoundEngineSetListeners,
    // Seeks inside all playing objects that are referenced in Play Actions of the specified Event. See <tt>AK::SoundEngine::SeekOnEvent</tt>.
    #[strum(serialize = "ak.soundengine.seekOnEvent")]
    SoundEngineSeekOnEvent,
    // Unregisters a game object. Registering a game object twice does nothing. Unregistering it once unregisters it no matter how many times it has been registered. Unregistering a game object while it is in use is allowed, but the control over the parameters of this game object is lost. For example, say a sound associated with this game object is a 3D moving sound. It stops moving when the game object is unregistered, and there is no way to regain control over the game object. See <tt>AK::SoundEngine::UnregisterGameObj</tt>.
    #[strum(serialize = "ak.soundengine.unregisterGameObj")]
    SoundEngineUnregisterGameObj,
    // Retrieves the list of topics to which a client can subscribe.
    #[strum(serialize = "ak.wwise.waapi.getTopics")]
    WwiseWaapiGetTopics,
    // Retrieves the list of functions.
    #[strum(serialize = "ak.wwise.waapi.getFunctions")]
    WwiseWaapiGetFunctions,
    // Retrieves the JSON schema of a Waapi URI.
    #[strum(serialize = "ak.wwise.waapi.getSchema")]
    WwiseWaapiGetSchema,
    // Opens a project, specified by path. Please refer to \ref ak_wwise_core_project_loaded for further explanations on how to be notified when the operation has completed.
    #[strum(serialize = "ak.wwise.ui.project.open")]
    WwiseUiProjectOpen,
    // Closes the current project.
    #[strum(serialize = "ak.wwise.ui.project.close")]
    WwiseUiProjectClose,
    // Saves the current project.
    #[strum(serialize = "ak.wwise.core.project.save")]
    WwiseCoreProjectSave,
    // Renames an object.
    #[strum(serialize = "ak.wwise.core.object.setName")]
    WwiseCoreObjectSetName,
    // Sets an object's reference value. Refer to \ref wobjects_index for more information on the references available on each object type.
    #[strum(serialize = "ak.wwise.core.object.setReference")]
    WwiseCoreObjectSetReference,
    // Sets a property value of an object for a specific platform. Refer to \ref wobjects_index for more information on the properties available on each object type. Refer to \ref ak_wwise_core_object_setreference to set a reference to an object.
    #[strum(serialize = "ak.wwise.core.object.setProperty")]
    WwiseCoreObjectSetProperty,
    // Sets the randomizer values of a property of an object for a specific platform. Refer to \ref wobjects_index for more information on the properties available on each object type.
    #[strum(serialize = "ak.wwise.core.object.setRandomizer")]
    WwiseCoreObjectSetRandomizer,
    // Sets the object's notes.
    #[strum(serialize = "ak.wwise.core.object.setNotes")]
    WwiseCoreObjectSetNotes,
    // Executes a command. Some commands can take a list of objects as parameters. Refer to \ref globalcommandsids for the available commands.
    #[strum(serialize = "ak.wwise.ui.commands.execute")]
    WwiseUiCommandsExecute,
    // Gets the list of commands.
    #[strum(serialize = "ak.wwise.ui.commands.getCommands")]
    WwiseUiCommandsGetCommands,
    // Retrieves the list of objects currently selected by the user in the active view.
    #[strum(serialize = "ak.wwise.ui.getSelectedObjects")]
    WwiseUiGetSelectedObjects,
    // Gets the specified attenuation curve for a given attenuation object.
    #[strum(serialize = "ak.wwise.core.object.getAttenuationCurve")]
    WwiseCoreObjectGetAttenuationCurve,
    // Sets the specified attenuation curve for a given attenuation object.
    #[strum(serialize = "ak.wwise.core.object.setAttenuationCurve")]
    WwiseCoreObjectSetAttenuationCurve,
    // Assigns a Switch Container's child to a Switch. This is the equivalent of doing a drag&drop of the child to a state in the Assigned Objects view. The child is always added at the end for each state.
    #[strum(serialize = "ak.wwise.core.switchContainer.addAssignment")]
    WwiseCoreSwitchContainerAddAssignment,
    // Removes an assignment between a Switch Container's child and a State.
    #[strum(serialize = "ak.wwise.core.switchContainer.removeAssignment")]
    WwiseCoreSwitchContainerRemoveAssignment,
    // Returns the list of assignments between a Switch Container's children and states.
    #[strum(serialize = "ak.wwise.core.switchContainer.getAssignments")]
    WwiseCoreSwitchContainerGetAssignments,
    // Creates an object of type 'type', as a child of 'parent'. Refer to \ref waapi_import for more information about creating objects. Also refer to \ref ak_wwise_core_audio_import to import audio files to Wwise.
    #[strum(serialize = "ak.wwise.core.object.create")]
    WwiseCoreObjectCreate,
    // Moves an object to the given parent. Returns the moved object.
    #[strum(serialize = "ak.wwise.core.object.move")]
    WwiseCoreObjectMove,
    // Copies an object to the given parent.
    #[strum(serialize = "ak.wwise.core.object.copy")]
    WwiseCoreObjectCopy,
    // Deletes the specified object.
    #[strum(serialize = "ak.wwise.core.object.delete")]
    WwiseCoreObjectDelete,
    // Performs a query and returns specified data for each object in the query result. The query can specify either a 'waql' argument or a 'from' argument with an optional 'transform' argument. Refer to \ref waql_introduction or \ref waapi_query for more information.
    #[strum(serialize = "ak.wwise.core.object.get")]
    WwiseCoreObjectGet,
    // Creates Wwise objects and imports audio files. This function uses the same importation processor available through the Tab Delimited import in the Audio File Importer. The function returns an array of all objects created, replaced or re-used. Use the options to specify how the objects are returned. For more information, refer to \ref waapi_import.
    #[strum(serialize = "ak.wwise.core.audio.import")]
    WwiseCoreAudioImport,
    // Scripted object creation and audio file import from a tab-delimited file.
    #[strum(serialize = "ak.wwise.core.audio.importTabDelimited")]
    WwiseCoreAudioImportTabDelimited,
    // Connects the Wwise Authoring application to a Wwise Sound Engine running executable. The host must be running code with communication enabled. If only "host" is provided, Wwise connects to the first Sound Engine instance found. To distinguish between different instances, you can also provide the name of the application to connect to.
    #[strum(serialize = "ak.wwise.core.remote.connect")]
    WwiseCoreRemoteConnect,
    // Disconnects the Wwise Authoring application from a connected Wwise Sound Engine running executable.
    #[strum(serialize = "ak.wwise.core.remote.disconnect")]
    WwiseCoreRemoteDisconnect,
    // Retrieves all consoles available for connecting Wwise Authoring to a Sound Engine instance.
    #[strum(serialize = "ak.wwise.core.remote.getAvailableConsoles")]
    WwiseCoreRemoteGetAvailableConsoles,
    // Retrieves the connection status.
    #[strum(serialize = "ak.wwise.core.remote.getConnectionStatus")]
    WwiseCoreRemoteGetConnectionStatus,
    // Begins an undo group. Make sure to call \ref ak_wwise_core_undo_endgroup exactly once for every ak.wwise.core.beginUndoGroup call you make. Calls to ak.wwise.core.undo.beginGroup can be nested.
    #[strum(serialize = "ak.wwise.core.undo.beginGroup")]
    WwiseCoreUndoBeginGroup,
    // Cancels the last undo group. Please note that this does not revert the operations made since the last \ref ak_wwise_core_undo_begingroup call.
    #[strum(serialize = "ak.wwise.core.undo.cancelGroup")]
    WwiseCoreUndoCancelGroup,
    // Ends the last undo group.
    #[strum(serialize = "ak.wwise.core.undo.endGroup")]
    WwiseCoreUndoEndGroup,
    // Retrieves the list of all object types registered in Wwise's object model. This function returns the equivalent of \ref wobjects_index .
    // \deprecated in favor of ak.wwise.core.object.getTypes
    #[strum(to_string = "ak.wwise.core.plugin.getList")]
    WwiseCorePluginGetList,
    // Retrieves the list of all object types registered in Wwise's object model. This function returns the equivalent of \ref wobjects_index .
    #[strum(to_string = "ak.wwise.core.object.getTypes")]
    WwiseCoreObjectGetTypes,
    // etrieves information about an object property.
    // \deprecated in favor of ak.wwise.core.object.getPropertyInfo
    #[strum(to_string = "ak.wwise.core.plugin.getProperty")]
    WwiseCorePluginGetProperty,
    // Retrieves information about an object property.
    #[strum(to_string = "ak.wwise.core.object.getPropertyInfo")]
    WwiseCoreObjectGetPropertyInfo,
    // Retrieves the list of property and reference names for an object.
    // \deprecated in favor of ak.wwise.core.object.getPropertyAndReferenceNames
    #[strum(to_string = "ak.wwise.core.plugin.getProperties")]
    WwiseCorePluginGetProperties,
    // Retrieves the list of property and reference names for an object.
    // \deprecated in favor of ak.wwise.core.object.getPropertyAndReferenceNames
    #[strum(to_string = "ak.wwise.core.object.getPropertyNames")]
    WwiseCoreObjectGetPropertyNames,
    // Retrieves the list of property and reference names for an object.
    #[strum(to_string = "ak.wwise.core.object.getPropertyAndReferenceNames")]
    WwiseCoreObjectGetPropertyAndReferenceNames,
    // Returns true if a property is enabled based on the values of the properties it depends on.
    #[strum(to_string = "ak.wwise.core.object.isPropertyEnabled")]
    WwiseCoreObjectIsPropertyEnabled,
    // Enables debug assertions. Every call to enableAsserts with 'false' increments the ref count. Calling with true decrements the ref count. This is only available with Debug builds.
    #[strum(to_string = "ak.wwise.debug.enableAsserts")]
    WwiseDebugEnableAsserts,
    // Private use only.
    #[strum(to_string = "ak.wwise.debug.testAssert")]
    WwiseDebugTestAssert,
    // Private use only.
    #[strum(to_string = "ak.wwise.debug.testCrash")]
    WwiseDebugTestCrash,
    // Enables or disables the automation mode for Wwise. This reduces the potential interruptions caused by message boxes and dialogs. For instance, enabling the automation mode silently accepts: project migration, project load log, EULA acceptance, project licence display and generic message boxes.
    #[strum(to_string = "ak.wwise.debug.enableAutomationMode")]
    WwiseDebugEnableAutomationMode,
    // Captures a part of the Wwise UI relative to a view.
    #[strum(to_string = "ak.wwise.ui.captureScreen")]
    WwiseUiCaptureScreen,
    // Retrieves a SoundBank's inclusion list.
    #[strum(to_string = "ak.wwise.core.soundbank.getInclusions")]
    WwiseCoreSoundbankGetInclusions,
    // Modifies a SoundBank's inclusion list. The 'operation' argument determines how the 'inclusions' argument modifies the SoundBank's inclusion list; 'inclusions' may be added to / removed from / replace the SoundBank's inclusion list.
    #[strum(to_string = "ak.wwise.core.soundbank.setInclusions")]
    WwiseCoreSoundbankSetInclusions,
    // Generate a list of SoundBank with import definition defined in the WAAPI request. If you choose to not write the soundbanks to disk, subscribe to \ref ak_wwise_core_soundbank_generated to get SoundBank structure info and the bank data as base64.
    #[strum(to_string = "ak.wwise.core.soundbank.generate")]
    WwiseCoreSoundbankGenerate,
    // Migrate and save the project.
    #[strum(to_string = "ak.wwise.cli.migrate")]
    WwiseCliMigrate,
    // Imports a tab-delimited file to create and modify different object hierarchies. The project is automatically migrated (if required). It is also automatically saved following the import.
    #[strum(to_string = "ak.wwise.cli.tabDelimitedImport")]
    WwiseCliTabDelimitedImport,
    // Starts a command-line Wwise Authoring API server, to which client applications, using the Wwise Authoring API, can connect.
    #[strum(to_string = "ak.wwise.cli.waapiServer")]
    WwiseCliWaapiServer,
    // Creates a blank new project. The project must not already exist. If the folder does not exist, it is created.
    #[strum(to_string = "ak.wwise.cli.createNewProject")]
    WwiseCliCreateNewProject,
    // Dump the objects model of a project as a JSON file.
    #[strum(to_string = "ak.wwise.cli.dumpObjects")]
    WwiseCliDumpObjects,
    // Adds a new platform to a project. The platform must not already exist.
    #[strum(to_string = "ak.wwise.cli.addNewPlatform")]
    WwiseCliAddNewPlatform,
    // External Sources conversion. Converts the external sources files for the specified project. Optionally, additional WSOURCES can be specified. External Sources are a special type of source that you can put in a Sound object in Wwise. It indicates that the real sound data will be provided at run time. While External Source conversion is also triggered by SoundBank generation, this operation can be used to process sources not contained in the Wwise Project. Please refer to Wwise SDK help page "Integrating External Sources".
    #[strum(to_string = "ak.wwise.cli.convertExternalSource")]
    WwiseCliConvertExternalSource,
    // SoundBank generation. SoundBank generation is performed according to the settings stored in the project. Custom user settings are ignored when SoundBank generation is launched from the command line. However, some of these settings can be overridden from the command line.
    #[strum(to_string = "ak.wwise.cli.generateSoundbank")]
    WwiseCliGenerateSoundbank,
    // Moves the project's media IDs from its work units (.wwu) to a single file, <project-name>.mediaid.  This command will force a save of all the project's work units.
    #[strum(to_string = "ak.wwise.cli.moveMediaIdsToSingleFile")]
    WwiseCliMoveMediaIdsToSingleFile,
    // Moves the project's media IDs from a single xml file <project-name>.mediaid to its work units (.wwu).  This command will force a save of all the project's work units.
    #[strum(to_string = "ak.wwise.cli.moveMediaIdsToWorkUnits")]
    WwiseCliMoveMediaIdsToWorkUnits,
    // Loads the project and updates the contents of <project-name>.mediaid, if it exists.
    #[strum(to_string = "ak.wwise.cli.updateMediaIdsInSingleFile")]
    WwiseCliUpdateMediaIdsInSingleFile,
    // Retrieves the latest log for a specific channel. Refer to \ref ak_wwise_core_log_itemadded to be notified when a item is added to the log.
    #[strum(to_string = "ak.wwise.core.log.get")]
    WwiseCoreLogGet,
    // Creates a transport object for the given Wwise object. The return transport object can be used to play, stop, pause and resume the Wwise object via the other transport functions.
    #[strum(to_string = "ak.wwise.core.transport.create")]
    WwiseCoreTransportCreate,
    // Destroys the given transport object.
    #[strum(to_string = "ak.wwise.core.transport.destroy")]
    WwiseCoreTransportDestroy,
    // Gets the state of the given transport object.
    #[strum(to_string = "ak.wwise.core.transport.getState")]
    WwiseCoreTransportGetState,
    // Returns the list of transport objects.
    #[strum(to_string = "ak.wwise.core.transport.getList")]
    WwiseCoreTransportGetList,
    // Executes an action on the given transport object, or all transport objects if none is specified.
    #[strum(to_string = "ak.wwise.core.transport.executeAction")]
    WwiseCoreTransportExecuteAction,
    // Gets the min/max peak pairs, in the given region of an audio source, as a collection of binary strings (one per channel). The strings are base-64 encoded, 16-bit signed int arrays, with min and max values being interleaved. If getCrossChannelPeaks is true, only one binary string represents the peaks across all channels globally.
    #[strum(to_string = "ak.wwise.core.audioSourcePeaks.getMinMaxPeaksInRegion")]
    WwiseCoreAudioSourcePeaksGetMinMaxPeaksInRegion,
    // Gets the min/max peak pairs in the entire trimmed region of an audio source, for each channel, as an array of binary strings (one per channel). The strings are base-64 encoded, 16-bit signed int arrays, with min and max values being interleaved. If getCrossChannelPeaks is true, there is only one binary string representing peaks across all channels globally.
    #[strum(to_string = "ak.wwise.core.audioSourcePeaks.getMinMaxPeaksInTrimmedRegion")]
    WwiseCoreAudioSourcePeaksGetMinMaxPeaksInTrimmedRegion,
    // Registers an array of add-on commands. Registered commands remain until the Wwise process is terminated. Refer to \ref defining_custom_commands for more information about registering commands. Also refer to \ref ak_wwise_ui_commands_executed.
    #[strum(to_string = "ak.wwise.ui.commands.register")]
    WwiseUiCommandsRegister,
    // Unregisters an array of add-on UI commands.
    #[strum(to_string = "ak.wwise.ui.commands.unregister")]
    WwiseUiCommandsUnregister,
    // Retrieves the Audio Objects at a specific profiler capture time.
    #[strum(to_string = "ak.wwise.core.profiler.getAudioObjects")]
    WwiseCoreProfilerGetAudioObjects,
    // Retrieves the voices at a specific profiler capture time.
    #[strum(to_string = "ak.wwise.core.profiler.getVoices")]
    WwiseCoreProfilerGetVoices,
    // Retrieves active RTPCs at a specific profiler capture time.
    #[strum(to_string = "ak.wwise.core.profiler.getRTPCs")]
    WwiseCoreProfilerGetRTPCs,
    // Retrieves the busses at a specific profiler capture time.
    #[strum(to_string = "ak.wwise.core.profiler.getBusses")]
    WwiseCoreProfilerGetBusses,
    // Retrieves all parameters affecting voice volume, highpass and lowpass for a voice path, resolved from pipeline IDs.
    #[strum(to_string = "ak.wwise.core.profiler.getVoiceContributions")]
    WwiseCoreProfilerGetVoiceContributions,
    // Specifies the type of data you want to capture. Overrides the user's profiler settings.
    #[strum(to_string = "ak.wwise.core.profiler.enableProfilerData")]
    WwiseCoreProfilerEnableProfilerData,
    // Returns the current time of the specified profiler cursor, in milliseconds.
    #[strum(to_string = "ak.wwise.core.profiler.getCursorTime")]
    WwiseCoreProfilerGetCursorTime,
    // Starts the profiler capture and returns the time at the beginning of the capture, in milliseconds.
    #[strum(to_string = "ak.wwise.core.profiler.startCapture")]
    WwiseCoreProfilerStartCapture,
    // Stops the profiler capture and returns the time at the end of the capture, in milliseconds.
    #[strum(to_string = "ak.wwise.core.profiler.stopCapture")]
    WwiseCoreProfilerStopCapture,
    // Sent at the end of an import operation.
    #[strum(to_string = "ak.wwise.core.audio.imported")]
    WwiseCoreAudioImported,
    // Sent when an object reference is changed.
    #[strum(to_string = "ak.wwise.core.object.referenceChanged")]
    WwiseCoreObjectReferenceChanged,
    // Sent when an assignment is added to a Switch Container.
    #[strum(to_string = "ak.wwise.core.switchContainer.assignmentAdded")]
    WwiseCoreSwitchContainerAssignmentAdded,
    // Sent when an assignment is removed from a Switch Container.
    #[strum(to_string = "ak.wwise.core.switchContainer.assignmentRemoved")]
    WwiseCoreSwitchContainerAssignmentRemoved,
    // Sent when an object is renamed. Publishes the renamed object.
    #[strum(to_string = "ak.wwise.core.object.nameChanged")]
    WwiseCoreObjectNameChanged,
    // Sent when the object's notes are changed.
    #[strum(to_string = "ak.wwise.core.object.notesChanged")]
    WwiseCoreObjectNotesChanged,
    // Sent when an object is created.
    #[strum(to_string = "ak.wwise.core.object.created")]
    WwiseCoreObjectCreated,
    // Sent prior to an object's deletion.
    #[strum(to_string = "ak.wwise.core.object.preDeleted")]
    WwiseCoreObjectPreDeleted,
    // Sent following an object's deletion.
    #[strum(to_string = "ak.wwise.core.object.postDeleted")]
    WwiseCoreObjectPostDeleted,
    // Sent when an object is added as a child to another object.
    #[strum(to_string = "ak.wwise.core.object.childAdded")]
    WwiseCoreObjectChildAdded,
    // Sent when an object is removed from the children of another object.
    #[strum(to_string = "ak.wwise.core.object.childRemoved")]
    WwiseCoreObjectChildRemoved,
    // Sent when an attenuation curve is changed.
    #[strum(to_string = "ak.wwise.core.object.curveChanged")]
    WwiseCoreObjectCurveChanged,
    // Sent when an attenuation curve is changed.
    #[strum(to_string = "ak.wwise.core.object.attenuationCurveChanged")]
    WwiseCoreObjectAttenuationCurveChanged,
    // Sent when an attenuation curve's link/unlink is changed.
    #[strum(to_string = "ak.wwise.core.object.attenuationCurveLinkChanged")]
    WwiseCoreObjectAttenuationCurveLinkChanged,
    // Sent when the watched property of an object changes.
    #[strum(to_string = "ak.wwise.core.object.propertyChanged")]
    WwiseCoreObjectPropertyChanged,
    // Sent when a single SoundBank is generated. This could be sent multiple times during SoundBank generation, for every SoundBank generated and for every platform. To generate SoundBanks, refer to \ref ak_wwise_core_soundbank_generate or \ref ak_wwise_ui_commands_execute with one of the SoundBank generation commands. Refer to \ref globalcommandsids for the list of commands.
    #[strum(to_string = "ak.wwise.core.soundbank.generated")]
    WwiseCoreSoundbankGenerated,
    // Sent when all soundbanks are generated.
    #[strum(to_string = "ak.wwise.core.soundbank.generationDone")]
    WwiseCoreSoundbankGenerationDone,
    // Sent when an item is added to the log. This could be used to retrieve items added to the SoundBank generation log. To retrieve the complete log, refer to \ref ak_wwise_core_log_get.
    #[strum(to_string = "ak.wwise.core.log.itemAdded")]
    WwiseCoreLogItemAdded,
    // Sent when the selection changes in the project.
    #[strum(to_string = "ak.wwise.ui.selectionChanged")]
    WwiseUiSelectionChanged,
    // Sent when the project has been successfully loaded.
    #[strum(to_string = "ak.wwise.core.project.loaded")]
    WwiseCoreProjectLoaded,
    // Sent when the project begins closing.
    #[strum(to_string = "ak.wwise.core.project.preClosed")]
    WwiseCoreProjectPreClosed,
    // Sent when the after the project is completely closed.
    #[strum(to_string = "ak.wwise.core.project.postClosed")]
    WwiseCoreProjectPostClosed,
    // Sent when the project has been saved.
    #[strum(to_string = "ak.wwise.core.project.saved")]
    WwiseCoreProjectSaved,
    // Sent when the transport's state has changed.
    #[strum(to_string = "ak.wwise.core.transport.stateChanged")]
    WwiseCoreTransportStateChanged,
    // Sent when an assert has failed. This is only available in Debug builds.
    #[strum(to_string = "ak.wwise.debug.assertFailed")]
    WwiseDebugAssertFailed,
    // Sent when a command is executed. The objects for which the command is executed are sent in the publication.
    #[strum(to_string = "ak.wwise.ui.commands.executed")]
    WwiseUiCommandsExecuted,
    // Sent when a new entry is added to the capture log. Note that all entries are being sent. No filtering is applied as opposed to the Capture Log view.
    #[strum(to_string = "ak.wwise.core.profiler.captureLog.itemAdded")]
    WwiseCoreProfilerCaptureLogItemAdded,
}// 
