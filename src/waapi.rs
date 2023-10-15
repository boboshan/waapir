pub mod ak {
    pub mod wwise {
        pub mod core {
            // Retrieve global Wwise information.
            pub const GET_INFO: &str = "ak.wwise.core.getInfo";
            // Retrieve information about the current project opened, including platforms, languages, and project directories.
            pub const GET_PROJECT_INFO: &str = "ak.wwise.core.getProjectInfo";
            pub mod project {
                // Saves the current project.
                pub const SAVE: &str = "ak.wwise.core.project.save";
                // Sent when the project has been successfully loaded.
                pub const LOADED: &str = "ak.wwise.core.project.loaded";
                // Sent when the project begins closing.
                pub const PRE_CLOSED: &str = "ak.wwise.core.project.preClosed";
                // Sent when the after the project is completely closed.
                pub const POST_CLOSED: &str = "ak.wwise.core.project.postClosed";
                // Sent when the project has been saved.
                pub const SAVED: &str = "ak.wwise.core.project.saved";
            }

            pub mod object {
                // Renames an object.
                pub const SET_NAME: &str = "ak.wwise.core.object.setName";
                // Sets an object's reference value. Refer to wobjects_index for more information on the references available on each object type.
                pub const SET_REFERENCE: &str = "ak.wwise.core.object.setReference";
                // Sets a property value of an object for a specific platform. Refer to wobjects_index for more information on the properties available on each object type. Refer to ak_wwise_core_object_setreference to set a reference to an object. Refer to ak_wwise_core_object_get to obtain the value of a property for an object.
                pub const SET_PROPERTY: &str = "ak.wwise.core.object.setProperty";
                // Sets the randomizer values of a property of an object for a specific platform. Refer to wobjects_index for more information on the properties available on each object type.
                pub const SET_RANDOMIZER: &str = "ak.wwise.core.object.setRandomizer";
                // Sets the object's notes.
                pub const SET_NOTES: &str = "ak.wwise.core.object.setNotes";
                // Gets the specified attenuation curve for a given attenuation object.
                pub const GET_ATTENUATION_CURVE: &str = "ak.wwise.core.object.getAttenuationCurve";
                // Sets the specified attenuation curve for a given attenuation object.
                pub const SET_ATTENUATION_CURVE: &str = "ak.wwise.core.object.setAttenuationCurve";
                // Creates an object of type 'type', as a child of 'parent'. Refer to waapi_import for more information about creating objects. Also refer to ak_wwise_core_audio_import to import audio files to Wwise.
                pub const CREATE: &str = "ak.wwise.core.object.create";
                // Allows for batch processing of the following operations: Object creation in a child hierarchy, Object creation in a list, Setting name, notes, properties and references. Refer to waapi_import for more information about creating objects. Also refer to ak_wwise_core_audio_import to import audio files to Wwise.
                pub const SET: &str = "ak.wwise.core.object.set";
                // Moves an object to the given parent. Returns the moved object.
                pub const MOVE: &str = "ak.wwise.core.object.move";
                // Copies an object to the given parent.
                pub const COPY: &str = "ak.wwise.core.object.copy";
                // Deletes the specified object.
                pub const DELETE: &str = "ak.wwise.core.object.delete";
                // Performs a query and returns the data, as specified in the options, for each object in the query result. The query can specify either a 'waql' argument or a 'from' argument with an optional 'transform' argument. Refer to waql_introduction or waapi_query for more information. Refer to waapi_query_return to learn about options.
                pub const GET: &str = "ak.wwise.core.object.get";
                // Compares properties and lists of the source object with those in the target object.
                pub const DIFF: &str = "ak.wwise.core.object.diff";
                // Pastes properties, references and lists from one object to any number of target objects. Only those properties, references and lists which differ between source and target are pasted. Refer to wobjects_index for more information on the properties, references and lists available on each object type.
                pub const PASTE_PROPERTIES: &str = "ak.wwise.core.object.pasteProperties";
                // Retrieves the list of all object types registered in Wwise's object model. This function returns the equivalent of wobjects_index.
                pub const GET_TYPES: &str = "ak.wwise.core.object.getTypes";
                // Retrieves information about an object property. Note that this function does not return the value of a property. To retrieve the value of a property, refer to ak_wwise_core_object_get and waapi_query_return.
                pub const GET_PROPERTY_INFO: &str = "ak.wwise.core.object.getPropertyInfo";
                // Retrieves the list of property and reference names for an object.
                pub const GET_PROPERTY_NAMES: &str = "ak.wwise.core.object.getPropertyNames";
                // Retrieves the list of property and reference names for an object.
                pub const GET_PROPERTY_AND_REFERENCE_NAMES: &str =
                    "ak.wwise.core.object.getPropertyAndReferenceNames";
                // Returns true if a property is enabled based on the values of the properties it depends on.
                pub const IS_PROPERTY_ENABLED: &str = "ak.wwise.core.object.isPropertyEnabled";
                // Sent when an object reference is changed.
                pub const REFERENCE_CHANGED: &str = "ak.wwise.core.object.referenceChanged";
                // Sent when an object is renamed. Publishes the renamed object.
                pub const NAME_CHANGED: &str = "ak.wwise.core.object.nameChanged";
                // Sent when the object's notes are changed.
                pub const NOTES_CHANGED: &str = "ak.wwise.core.object.notesChanged";
                // Sent when an object is created.
                pub const CREATED: &str = "ak.wwise.core.object.created";
                // Sent prior to an object's deletion.
                pub const PRE_DELETED: &str = "ak.wwise.core.object.preDeleted";
                // Sent following an object's deletion.
                pub const POST_DELETED: &str = "ak.wwise.core.object.postDeleted";
                // Sent when an object is added as a child to another object.
                pub const CHILD_ADDED: &str = "ak.wwise.core.object.childAdded";
                // Sent when an object is removed from the children of another object.
                pub const CHILD_REMOVED: &str = "ak.wwise.core.object.childRemoved";
                // Sent when one or many curves are changed.
                pub const CURVE_CHANGED: &str = "ak.wwise.core.object.curveChanged";
                // Sent when an attenuation curve is changed.
                pub const ATTENUATION_CURVE_CHANGED: &str =
                    "ak.wwise.core.object.attenuationCurveChanged";
                // Sent when an attenuation curve's link/unlink is changed.
                pub const ATTENUATION_CURVE_LINK_CHANGED: &str =
                    "ak.wwise.core.object.attenuationCurveLinkChanged";
                // Sent when the watched property of an object changes.
                pub const PROPERTY_CHANGED: &str = "ak.wwise.core.object.propertyChanged";
            }
            pub mod switch_container {
                // Assigns a Switch Container's child to a Switch. This is the equivalent of doing a drag&drop of the child to a state in the Assigned Objects view. The child is always added at the end for each state.
                pub const ADD_ASSIGNMENT: &str = "ak.wwise.core.switchContainer.addAssignment";
                // Removes an assignment between a Switch Container's child and a State.
                pub const REMOVE_ASSIGNMENT: &str = "ak.wwise.core.switchContainer.removeAssignment";
                // Returns the list of assignments between a Switch Container's children and states.
                pub const GET_ASSIGNMENTS: &str = "ak.wwise.core.switchContainer.getAssignments";
                // Sent when an assignment is added to a Switch Container.
                pub const ASSIGNMENT_ADDED: &str = "ak.wwise.core.switchContainer.assignmentAdded";
                // Sent when an assignment is removed from a Switch Container.
                pub const ASSIGNMENT_REMOVED: &str = "ak.wwise.core.switchContainer.assignmentRemoved";
            }
            
            pub mod audio {
                // Creates Wwise objects and imports audio files. This function uses the same importation processor available through the Tab Delimited import in the Audio File Importer. The function returns an array of all objects created, replaced or re-used. Use the options to specify how the objects are returned. For more information, refer to \ref waapi_import.
                pub const IMPORT: &str = "ak.wwise.core.audio.import";
                // Scripted object creation and audio file import from a tab-delimited file.
                pub const IMPORT_TAB_DELIMITED: &str = "ak.wwise.core.audio.importTabDelimited";
                // Sent at the end of an import operation.
                pub const IMPORTED: &str = "ak.wwise.core.audio.imported";
            }
            
            pub mod remote {
                // Connects the Wwise Authoring application to a Wwise Sound Engine running executable. The host must be running code with communication enabled. If only "host" is provided, Wwise connects to the first Sound Engine instance found. To distinguish between different instances, you can also provide the name of the application to connect to.
                pub const CONNECT: &str = "ak.wwise.core.remote.connect";
                // Disconnects the Wwise Authoring application from a connected Wwise Sound Engine running executable.
                pub const DISCONNECT: &str = "ak.wwise.core.remote.disconnect";
                // Retrieves all consoles available for connecting Wwise Authoring to a Sound Engine instance.
                pub const GET_AVAILABLE_CONSOLES: &str = "ak.wwise.core.remote.getAvailableConsoles";
                // Retrieves the connection status.
                pub const GET_CONNECTION_STATUS: &str = "ak.wwise.core.remote.getConnectionStatus";
            }
            
            pub mod undo {
                // Begins an undo group. Make sure to call \ref ak_wwise_core_undo_endgroup exactly once for every ak.wwise.core.beginUndoGroup call you make. Calls to ak.wwise.core.undo.beginGroup can be nested. When closing a WAMP session, a check is made to ensure that all undo groups are closed. If not, a cancelGroup is called for each of the groups still open.
                pub const BEGIN_GROUP: &str = "ak.wwise.core.undo.beginGroup";
                // Cancels the last undo group. Please note that this does not revert the operations made since the last \ref ak_wwise_core_undo_begingroup call.
                pub const CANCEL_GROUP: &str = "ak.wwise.core.undo.cancelGroup";
                // Ends the last undo group.
                pub const END_GROUP: &str = "ak.wwise.core.undo.endGroup";
                // Undoes the last operation in the Undo stack.
                pub const UNDO: &str = "ak.wwise.core.undo.undo";
            }
            
            pub mod plugin {
                // Retrieves the list of all object types registered in Wwise's object model. This function returns the equivalent of \ref wobjects_index .
                pub const GET_LIST: &str = "ak.wwise.core.plugin.getList";
                // Retrieves information about an object property. Note that this function does not return the value of a property. To retrieve the value of a property, refer to \ref ak_wwise_core_object_get and \ref waapi_query_return.
                pub const GET_PROPERTY: &str = "ak.wwise.core.plugin.getProperty";
                // Retrieves the list of property and reference names for an object.
                pub const GET_PROPERTIES: &str = "ak.wwise.core.plugin.getProperties";
            }
            
            pub mod soundbank {
                // Retrieves a SoundBank's inclusion list.
                pub const GET_INCLUSIONS: &str = "ak.wwise.core.soundbank.getInclusions";
                // Modifies a SoundBank's inclusion list. The 'operation' argument determines how the 'inclusions' argument modifies the SoundBank's inclusion list; 'inclusions' may be added to / removed from / replace the SoundBank's inclusion list.
                pub const SET_INCLUSIONS: &str = "ak.wwise.core.soundbank.setInclusions";
                // Generate a list of SoundBank with import definition defined in the WAAPI request. If you choose to not write the SoundBanks to disk, subscribe to \ref ak_wwise_core_soundbank_generated to get SoundBank structure info and the bank data as base64.
                pub const GENERATE: &str = "ak.wwise.core.soundbank.generate";
                // Imports SoundBank definitions from the specified file. Multiple files can be specified. See the WAAPI log for status messages.
                pub const PROCESS_DEFINITION_FILES: &str = "ak.wwise.core.soundbank.processDefinitionFiles";
                // Converts the external sources files for the project as detailed in the wsources file, and places them into either the default folder, or the folder specified by the output argument. External Sources are a special type of source that you can put in a Sound object in Wwise. It indicates that the real sound data will be provided at run time. While External Source conversion is also triggered by SoundBank generation, this operation can be used to process sources not contained in the Wwise Project. Please refer to Wwise SDK help page "Integrating External Sources".
                pub const CONVERT_EXTERNAL_SOURCES: &str = "ak.wwise.core.soundbank.convertExternalSources";
                // Sent when a single SoundBank is generated. This could be sent multiple times during SoundBank generation, for every SoundBank generated and for every platform. To generate SoundBanks, refer to \ref ak_wwise_core_soundbank_generate or \ref ak_wwise_ui_commands_execute with one of the SoundBank generation commands. Refer to \ref globalcommandsids for the list of commands.
                pub const GENERATED: &str = "ak.wwise.core.soundbank.generated";
                // Sent when all SoundBanks are generated.
                pub const GENERATION_DONE: &str = "ak.wwise.core.soundbank.generationDone";
            }
            
            pub mod sound {
                // Sets which version of the source is being used for the specified sound. Use \ref ak_wwise_core_object_get with the 'activeSource' return option to get the active source of a sound.
                pub const SET_ACTIVE_SOURCE: &str = "ak.wwise.core.sound.setActiveSource";
            }
            pub mod log {
                // Retrieves the latest log for a specific channel. Refer to \ref ak_wwise_core_log_itemadded to be notified when an item is added to the log.
                pub const GET: &str = "ak.wwise.core.log.get";
                // Sent when an item is added to the log. This could be used to retrieve items added to the SoundBank generation log. To retrieve the complete log, refer to \ref ak_wwise_core_log_get.
                pub const ITEM_ADDED: &str = "ak.wwise.core.log.itemAdded";
            }
            
            pub mod transport {
                // Creates a transport object for the given Wwise object. The return transport object can be used to play, stop, pause and resume the Wwise object via the other transport functions.
                pub const CREATE: &str = "ak.wwise.core.transport.create";
                // Prepare the object and its dependencies for playback. Use this function before calling PostEventSync or PostMIDIOnEventSync from IAkGlobalPluginContext.
                pub const PREPARE: &str = "ak.wwise.core.transport.prepare";
                // Destroys the given transport object.
                pub const DESTROY: &str = "ak.wwise.core.transport.destroy";
                // Gets the state of the given transport object.
                pub const GET_STATE: &str = "ak.wwise.core.transport.getState";
                // Returns the list of transport objects.
                pub const GET_LIST: &str = "ak.wwise.core.transport.getList";
                // Executes an action on the given transport object, or all transport objects if none is specified.
                pub const EXECUTE_ACTION: &str = "ak.wwise.core.transport.executeAction";
                // Sent when the transport's state has changed.
                pub const STATE_CHANGED: &str = "ak.wwise.core.transport.stateChanged";
            }
            
            pub mod audio_source_peaks {
                // Gets the min/max peak pairs, in the given region of an audio source, as a collection of binary strings (one per channel). The strings are base-64 encoded, 16-bit signed int arrays, with min and max values being interleaved. If getCrossChannelPeaks is true, only one binary string represents the peaks across all channels globally.
                pub const GET_MIN_MAX_PEAKS_IN_REGION: &str = "ak.wwise.core.audioSourcePeaks.getMinMaxPeaksInRegion";
                // Gets the min/max peak pairs in the entire trimmed region of an audio source, for each channel, as an array of binary strings (one per channel). The strings are base-64 encoded, 16-bit signed int arrays, with min and max values being interleaved. If getCrossChannelPeaks is true, there is only one binary string representing peaks across all channels globally.
                pub const GET_MIN_MAX_PEAKS_IN_TRIMMED_REGION: &str =
                    "ak.wwise.core.audioSourcePeaks.getMinMaxPeaksInTrimmedRegion";
            }
            
            pub mod profiler {
                // Retrieves the Audio Objects at a specific profiler capture time.
                pub const GET_AUDIO_OBJECTS: &str = "ak.wwise.core.profiler.getAudioObjects";
                // Retrieves the game objects at a specific profiler capture time.
                pub const GET_GAME_OBJECTS: &str = "ak.wwise.core.profiler.getGameObjects";
                // Retrieves the voices at a specific profiler capture time.
                pub const GET_VOICES: &str = "ak.wwise.core.profiler.getVoices";
                // Retrieves active RTPCs at a specific profiler capture time.
                pub const GET_RTPCS: &str = "ak.wwise.core.profiler.getRTPCs";
                // Retrieves the buses at a specific profiler capture time.
                pub const GET_BUSSES: &str = "ak.wwise.core.profiler.getBusses";
                // Retrieves all parameters affecting voice volume, highpass and lowpass for a voice path, resolved from pipeline IDs.
                pub const GET_VOICE_CONTRIBUTIONS: &str = "ak.wwise.core.profiler.getVoiceContributions";
                // Specifies the type of data you want to capture. Overrides the user's profiler settings.
                pub const ENABLE_PROFILER_DATA: &str = "ak.wwise.core.profiler.enableProfilerData";
                // Returns the current time of the specified profiler cursor, in milliseconds.
                pub const GET_CURSOR_TIME: &str = "ak.wwise.core.profiler.getCursorTime";
                // Starts the profiler capture and returns the time at the beginning of the capture, in milliseconds.
                pub const START_CAPTURE: &str = "ak.wwise.core.profiler.startCapture";
                // Stops the profiler capture and returns the time at the end of the capture, in milliseconds.
                pub const STOP_CAPTURE: &str = "ak.wwise.core.profiler.stopCapture";
            
                pub mod capture_log {
                    // Sent when a new entry is added to the capture log. Note that all entries are being sent. No filtering is applied as opposed to the Capture Log view.
                    pub const ITEM_ADDED: &str = "ak.wwise.core.profiler.captureLog.itemAdded";
                }
            
                // Sent when a game object has been registered.
                pub const GAME_OBJECT_REGISTERED: &str = "ak.wwise.core.profiler.gameObjectRegistered";
                // Sent when a game object has been unregistered.
                pub const GAME_OBJECT_UNREGISTERED: &str = "ak.wwise.core.profiler.gameObjectUnregistered";
                // Sent when the game objects have been reset, such as closing a connection to a game while profiling.
                pub const GAME_OBJECT_RESET: &str = "ak.wwise.core.profiler.gameObjectReset";
                // Sent when a state group state has been changed. This subscription does not require the profiler capture log to be started.
                pub const STATE_CHANGED: &str = "ak.wwise.core.profiler.stateChanged";
                // Sent when a switch group state has been changed. This function does not require the profiler capture log to be started.
                pub const SWITCH_CHANGED: &str = "ak.wwise.core.profiler.switchChanged";
            }
        }
        pub mod ui {
            // Bring Wwise main window to foreground. Refer to SetForegroundWindow and AllowSetForegroundWindow on MSDN for more information on the restrictions. Refer to ak.wwise.core.getInfo to obtain the Wwise process ID for AllowSetForegroundWindow.
            pub const BRING_TO_FOREGROUND: &str = "ak.wwise.ui.bringToForeground";
        
            pub mod project {
                // Opens a project, specified by path. Please refer to \ref ak_wwise_core_project_loaded for further explanations on how to be notified when the operation has completed.
                pub const OPEN: &str = "ak.wwise.ui.project.open";
                // Closes the current project.
                pub const CLOSE: &str = "ak.wwise.ui.project.close";
            }
        
            pub mod commands {
                // Executes a command. Some commands can take a list of objects as parameters. Refer to \ref globalcommandsids for the available commands.
                pub const EXECUTE: &str = "ak.wwise.ui.commands.execute";
                // Gets the list of commands.
                pub const GET_COMMANDS: &str = "ak.wwise.ui.commands.getCommands";
                // Registers an array of add-on commands. Registered commands remain until the Wwise process is terminated. Refer to \ref defining_custom_commands for more information about registering commands. Also refer to \ref ak_wwise_ui_commands_executed.
                pub const REGISTER: &str = "ak.wwise.ui.commands.register";
                // Unregisters an array of add-on UI commands.
                pub const UNREGISTER: &str = "ak.wwise.ui.commands.unregister";
                // Sent when a command is executed. The objects for which the command is executed are sent in the publication.
                pub const EXECUTED: &str = "ak.wwise.ui.commands.executed";
            }
        
            // Retrieves the list of objects currently selected by the user in the active view.
            pub const GET_SELECTED_OBJECTS: &str = "ak.wwise.ui.getSelectedObjects";
            // Captures a part of the Wwise UI relative to a view.
            pub const CAPTURE_SCREEN: &str = "ak.wwise.ui.captureScreen";
            // Sent when the selection changes in the project.
            pub const SELECTION_CHANGED: &str = "ak.wwise.ui.selectionChanged";
        }
        
        pub mod waapi {
            // Retrieves the list of topics to which a client can subscribe.
            pub const GET_TOPICS: &str = "ak.wwise.waapi.getTopics";
            // Retrieves the list of functions.
            pub const GET_FUNCTIONS: &str = "ak.wwise.waapi.getFunctions";
            // Retrieves the JSON schema of a Waapi URI.
            pub const GET_SCHEMA: &str = "ak.wwise.waapi.getSchema";
        }
        
        pub mod debug {
            // Enables debug assertions. Every call to enableAsserts with 'false' increments the ref count. Calling with true decrements the ref count. This is only available with Debug builds.
            pub const ENABLE_ASSERTS: &str = "ak.wwise.debug.enableAsserts";
            // Private use only.
            pub const TEST_ASSERT: &str = "ak.wwise.debug.testAssert";
            // Private use only.
            pub const TEST_CRASH: &str = "ak.wwise.debug.testCrash";
            // Enables or disables the automation mode for Wwise. This reduces the potential interruptions caused by message boxes and dialogs. For instance, enabling the automation mode silently accepts: project migration, project load log, EULA acceptance, project licence display and generic message boxes.
            pub const ENABLE_AUTOMATION_MODE: &str = "ak.wwise.debug.enableAutomationMode";
            // Sent when an assert has failed. This is only available in Debug builds.
            pub const ASSERT_FAILED: &str = "ak.wwise.debug.assertFailed";
        }
        
        pub mod cli {
            // Migrate and save the project.
            pub const MIGRATE: &str = "ak.wwise.cli.migrate";
            // Load the project and do nothing else. This is useful to see the log for verification purposes without actually migrating and saving.
            pub const VERIFY: &str = "ak.wwise.cli.verify";
            // Imports a tab-delimited file to create and modify different object hierarchies. The project is automatically migrated (if required). It is also automatically saved following the import.
            pub const TAB_DELIMITED_IMPORT: &str = "ak.wwise.cli.tabDelimitedImport";
            // Starts a command-line Wwise Authoring API server, to which client applications, using the Wwise Authoring API, can connect.
            pub const WAAPI_SERVER: &str = "ak.wwise.cli.waapiServer";
            // Creates a blank new project. The project must not already exist. If the folder does not exist, it is created.
            pub const CREATE_NEW_PROJECT: &str = "ak.wwise.cli.createNewProject";
            // Dump the objects model of a project as a JSON file.
            pub const DUMP_OBJECTS: &str = "ak.wwise.cli.dumpObjects";
            // Adds a new platform to a project. The platform must not already exist.
            pub const ADD_NEW_PLATFORM: &str = "ak.wwise.cli.addNewPlatform";
            // External Sources conversion. Converts the external sources files for the specified project. Optionally, additional WSOURCES can be specified. External Sources are a special type of source that you can put in a Sound object in Wwise. It indicates that the real sound data will be provided at run time. While External Source conversion is also triggered by SoundBank generation, this operation can be used to process sources not contained in the Wwise Project. Please refer to \ref integrating_external_sources.
            pub const CONVERT_EXTERNAL_SOURCE: &str = "ak.wwise.cli.convertExternalSource";
            // SoundBank generation. SoundBank generation is performed according to the settings stored in the project. Custom user settings are ignored when SoundBank generation is launched from the command line. However, some of these settings can be overridden from the command line.
            pub const GENERATE_SOUNDBANK: &str = "ak.wwise.cli.generateSoundbank";
            // Moves the project's media IDs from its work units (.wwu) to a single file, <project-name>.mediaid.  This command will force a save of all the project's work units.
            pub const MOVE_MEDIA_IDS_TO_SINGLE_FILE: &str = "ak.wwise.cli.moveMediaIdsToSingleFile";
            // Moves the project's media IDs from a single xml file <project-name>.mediaid to its work units (.wwu).  This command will force a save of all the project's work units.
            pub const MOVE_MEDIA_IDS_TO_WORK_UNITS: &str = "ak.wwise.cli.moveMediaIdsToWorkUnits";
            // Loads the project and updates the contents of <project-name>.mediaid, if it exists.
            pub const UPDATE_MEDIA_IDS_IN_SINGLE_FILE: &str = "ak.wwise.cli.updateMediaIdsInSingleFile";
        }
    }
    pub mod soundengine {
        // Asynchronously post an Event to the sound engine (by event ID). See <tt>AK::SoundEngine::PostEvent</tt>.
        pub const POST_EVENT: &str = "ak.soundengine.postEvent";
        // Executes an action on all nodes that are referenced in the specified event in a Play action. See <tt>AK::SoundEngine::ExecuteActionOnEvent</tt>.
        pub const EXECUTE_ACTION_ON_EVENT: &str = "ak.soundengine.executeActionOnEvent";
        // Register a game object. Registering a game object twice does nothing. Unregistering it once unregisters it no matter how many times it has been registered. See <tt>AK::SoundEngine::RegisterGameObj</tt>.
        pub const REGISTER_GAME_OBJ: &str = "ak.soundengine.registerGameObj";
        // Stops the current content, associated with the specified playing ID, from playing. See <tt>AK::SoundEngine::StopPlayingID</tt>.
        pub const STOP_PLAYING_ID: &str = "ak.soundengine.stopPlayingID";
        // Stop playing the current content associated with the specified game object ID. If no game object is specified, all sounds are stopped. See <tt>AK::SoundEngine::StopAll</tt>.
        pub const STOP_ALL: &str = "ak.soundengine.stopAll";
        // Display a message in the Profiler's Capture Log view.
        pub const POST_MSG_MONITOR: &str = "ak.soundengine.postMsgMonitor";
        // Set a game object's obstruction and occlusion levels. This function is used to affect how an object should be heard by a specific listener. See <tt>AK::SoundEngine::SetObjectObstructionAndOcclusion</tt>.
        pub const SET_OBJECT_OBSTRUCTION_AND_OCCLUSION: &str = "ak.soundengine.setObjectObstructionAndOcclusion";
        // Set the output bus volume (direct) to be used for the specified game object. See <tt>AK::SoundEngine::SetGameObjectOutputBusVolume</tt>.
        pub const SET_GAME_OBJECT_OUTPUT_BUS_VOLUME: &str = "ak.soundengine.setGameObjectOutputBusVolume";
        // Sets the Auxiliary Busses to route the specified game object. See <tt>AK::SoundEngine::SetGameObjectAuxSendValues</tt>.
        pub const SET_GAME_OBJECT_AUX_SEND_VALUES: &str = "ak.soundengine.setGameObjectAuxSendValues";
        // Posts the specified Trigger. See <tt>AK::SoundEngine::PostTrigger</tt>.
        pub const POST_TRIGGER: &str = "ak.soundengine.postTrigger";
        // Sets the State of a Switch Group. See <tt>AK::SoundEngine::SetSwitch</tt>.
        pub const SET_SWITCH: &str = "ak.soundengine.setSwitch";
        // Gets the current state of a Switch Group for a given Game Object.
        pub const GET_SWITCH: &str = "ak.soundengine.getSwitch";
        // Sets the State of a State Group. See <tt>AK::SoundEngine::SetState</tt>.
        pub const SET_STATE: &str = "ak.soundengine.setState";
        // Gets the current state of a State Group. When using setState just prior to getState, allow a brief delay (no more than 10ms) for the information to update in the sound engine.
        pub const GET_STATE: &str = "ak.soundengine.getState";
        // Resets the value of a real-time parameter control to its default value, as specified in the Wwise project. See <tt>AK::SoundEngine::ResetRTPCValue</tt>.
        pub const RESET_RTPC_VALUE: &str = "ak.soundengine.resetRTPCValue";
        // Sets the value of a real-time parameter control. See <tt>AK::SoundEngine::SetRTPCValue</tt>.
        pub const SET_RTPC_VALUE: &str = "ak.soundengine.setRTPCValue";
        // Sets a listener's spatialization parameters. This lets you define listener-specific volume offsets for each audio channel. See <tt>AK::SoundEngine::SetListenerSpatialization</tt>.
        pub const SET_LISTENER_SPATIALIZATION: &str = "ak.soundengine.setListenerSpatialization";
        // Sets multiple positions for a single game object. Setting multiple positions for a single game object is a way to simulate multiple emission sources while using the resources of only one voice. This can be used to simulate wall openings, area sounds, or multiple objects emitting the same sound in the same area. See <tt>AK::SoundEngine::SetMultiplePositions</tt>.
        pub const SET_MULTIPLE_POSITIONS: &str = "ak.soundengine.setMultiplePositions";
        // Sets the position of a game object. See <tt>AK::SoundEngine::SetPosition</tt>.
        pub const SET_POSITION: &str = "ak.soundengine.setPosition";
        // Sets the scaling factor of a game object. You can modify the attenuation computations on this game object to simulate sounds with a larger or smaller affected area. See <tt>AK::SoundEngine::SetScalingFactor</tt>.
        pub const SET_SCALING_FACTOR: &str = "ak.soundengine.setScalingFactor";
        // Sets the default active listeners for all subsequent game objects that are registered. See <tt>AK::SoundEngine::SetDefaultListeners</tt>.
        pub const SET_DEFAULT_LISTENERS: &str = "ak.soundengine.setDefaultListeners";
        // Sets a single game object's active listeners. By default, all new game objects have no listeners active, but this behavior can be overridden with <tt>SetDefaultListeners()</tt>. Inactive listeners are not computed. See <tt>AK::SoundEngine::SetListeners</tt>.
        pub const SET_LISTENERS: &str = "ak.soundengine.setListeners";
        // Seeks inside all playing objects that are referenced in Play Actions of the specified Event. See <tt>AK::SoundEngine::SeekOnEvent</tt>.
        pub const SEEK_ON_EVENT: &str = "ak.soundengine.seekOnEvent";
        // Unregisters a game object. Registering a game object twice does nothing. Unregistering it once unregisters it no matter how many times it has been registered. Unregistering a game object while it is in use is allowed, but the control over the parameters of this game object is lost. For example, say a sound associated with this game object is a 3D moving sound. It stops moving when the game object is unregistered, and there is no way to regain control over the game object. See <tt>AK::SoundEngine::UnregisterGameObj</tt>.
        pub const UNREGISTER_GAME_OBJ: &str = "ak.soundengine.unregisterGameObj";
    }
}