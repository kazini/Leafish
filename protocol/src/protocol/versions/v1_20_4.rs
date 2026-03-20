protocol_packet_ids!(
    handshake Handshaking {
        serverbound Serverbound {
            0x00 => Handshake
        }
        clientbound Clientbound {
        }
    }
    play Play {
        serverbound Serverbound {
            0x00 => TeleportConfirm
            0x01 => QueryBlockNBT
            0x02 => SetDifficulty
            // 0x03 => message_acknowledgement (no internal equivalent)
            // 0x04 => chat_command (no internal equivalent)
            0x05 => ChatMessage
            // 0x06 => chat_session_update (no internal equivalent)
            // 0x07 => chunk_batch_received (no internal equivalent)
            0x08 => ClientStatus
            0x09 => ClientSettings
            0x0a => TabComplete
            // 0x0b => configuration_acknowledged (no internal equivalent)
            0x0c => EnchantItem
            0x0d => ClickWindow
            0x0e => CloseWindow
            // 0x0f => set_slot_state (no internal equivalent)
            0x10 => PluginMessageServerbound
            0x11 => EditBook
            0x12 => QueryEntityNBT
            0x13 => UseEntity_Sneakflag
            0x14 => GenerateStructure
            0x15 => KeepAliveServerbound_i64
            0x16 => LockDifficulty
            0x17 => PlayerPosition
            0x18 => PlayerPositionLook
            0x19 => PlayerLook
            0x1a => Player
            0x1b => VehicleMove
            0x1c => SteerBoat
            0x1d => PickItem
            // 0x1e => ping_request (no internal equivalent)
            0x1f => CraftRecipeRequest
            0x20 => ClientAbilities_u8
            0x21 => PlayerDigging
            0x22 => PlayerAction
            0x23 => SteerVehicle
            // 0x24 => pong (no internal equivalent)
            0x25 => SetRecipeBookState
            0x26 => SetDisplayedRecipe
            0x27 => NameItem
            0x28 => ResourcePackStatus
            0x29 => AdvancementTab
            0x2a => SelectTrade
            0x2b => SetBeaconEffect
            0x2c => HeldItemChange
            0x2d => UpdateCommandBlock
            0x2e => UpdateCommandBlockMinecart
            0x2f => CreativeInventoryAction
            0x30 => UpdateJigsawBlock_Joint
            0x31 => UpdateStructureBlock
            0x32 => SetSign
            0x33 => ArmSwing
            0x34 => SpectateTeleport
            0x35 => PlayerBlockPlacement_insideblock
            0x36 => UseItem
        }
        clientbound Clientbound {
            // 0x00 => bundle_delimiter (no internal equivalent)
            0x01 => SpawnObject_VarInt
            0x02 => SpawnExperienceOrb
            0x03 => Animation
            0x04 => Statistics
            0x05 => AcknowledgePlayerDigging
            0x06 => BlockBreakAnimation
            0x07 => UpdateBlockEntity
            0x08 => BlockAction
            0x09 => BlockChange_VarInt
            0x0a => BossBar
            0x0b => ServerDifficulty_Locked
            // 0x0c => chunk_batch_finished (no internal equivalent)
            // 0x0d => chunk_batch_start (no internal equivalent)
            // 0x0e => chunk_biomes (no internal equivalent)
            // 0x0f => clear_titles (no internal equivalent)
            0x10 => TabCompleteReply
            0x11 => DeclareCommands
            0x12 => WindowClose
            0x13 => WindowItems
            0x14 => WindowProperty
            0x15 => WindowSetSlot
            0x16 => SetCooldown
            // 0x17 => chat_suggestions (no internal equivalent)
            0x18 => PluginMessageClientbound
            // 0x19 => damage_event (no internal equivalent)
            // 0x1a => hide_message (no internal equivalent)
            0x1b => Disconnect
            // 0x1c => profileless_chat (no internal equivalent)
            0x1d => EntityAction
            0x1e => Explosion
            0x1f => ChunkUnload
            0x20 => ChangeGameState
            0x21 => WindowOpenHorse
            // 0x22 => hurt_animation (no internal equivalent)
            // 0x23 => initialize_world_border (no internal equivalent)
            0x24 => KeepAliveClientbound_i64
            0x25 => ChunkData_Biomes3D_VarInt
            0x26 => Effect
            0x27 => Particle_f64
            0x28 => UpdateLight_WithTrust
            0x29 => JoinGame_WorldNames_IsHard
            0x2a => Maps
            0x2b => TradeList_WithRestock
            0x2c => EntityMove_i16
            0x2d => EntityLookAndMove_i16
            0x2e => EntityLook_VarInt
            0x2f => VehicleTeleport
            0x30 => OpenBook
            0x31 => WindowOpen_VarInt
            0x32 => SignEditorOpen
            // 0x33 => ping (no internal equivalent)
            // 0x34 => ping_response (no internal equivalent)
            0x35 => CraftRecipeResponse
            0x36 => PlayerAbilities
            // 0x37 => player_chat (no internal equivalent)
            // 0x38 => end_combat_event (no internal equivalent)
            // 0x39 => enter_combat_event (no internal equivalent)
            0x3a => CombatEvent
            // 0x3b => player_remove (no internal equivalent)
            0x3c => PlayerInfo
            0x3d => FacePlayer
            0x3e => TeleportPlayer_WithConfirm
            0x3f => UnlockRecipes_WithBlastSmoker
            0x40 => EntityDestroy
            0x41 => EntityRemoveEffect
            // 0x42 => reset_score (no internal equivalent)
            // 0x43 => remove_resource_pack (no internal equivalent)
            0x44 => ResourcePackSend
            0x45 => Respawn_NBT
            0x46 => EntityHeadLook
            0x47 => MultiBlockChange_Packed
            0x48 => SelectAdvancementTab
            // 0x49 => server_data (no internal equivalent)
            // 0x4a => action_bar (no internal equivalent)
            // 0x4b => world_border_center (no internal equivalent)
            // 0x4c => world_border_lerp_size (no internal equivalent)
            // 0x4d => world_border_size (no internal equivalent)
            // 0x4e => world_border_warning_delay (no internal equivalent)
            // 0x4f => world_border_warning_reach (no internal equivalent)
            0x50 => Camera
            0x51 => SetCurrentHotbarSlot
            0x52 => UpdateViewPosition
            0x53 => UpdateViewDistance
            0x54 => SpawnPosition
            0x55 => ScoreboardDisplay
            0x56 => EntityMetadata
            0x57 => EntityAttach
            0x58 => EntityVelocity
            0x59 => EntityEquipment_Array
            0x5a => SetExperience
            0x5b => UpdateHealth
            0x5c => ScoreboardObjective
            0x5d => SetPassengers
            0x5e => Teams_VarInt
            0x5f => UpdateScore
            // 0x60 => simulation_distance (no internal equivalent)
            // 0x61 => set_title_subtitle (no internal equivalent)
            0x62 => TimeUpdate
            // 0x63 => set_title_text (no internal equivalent)
            // 0x64 => set_title_time (no internal equivalent)
            0x65 => EntitySoundEffect
            0x66 => SoundEffect
            // 0x67 => start_configuration (no internal equivalent)
            0x68 => StopSound
            // 0x69 => system_chat (no internal equivalent)
            0x6a => PlayerListHeaderFooter
            0x6b => NBTQueryResponse
            0x6c => CollectItem
            0x6d => EntityTeleport_f64
            // 0x6e => set_ticking_state (no internal equivalent)
            // 0x6f => step_tick (no internal equivalent)
            0x70 => Advancements
            0x71 => EntityProperties
            0x72 => EntityEffect
            0x73 => DeclareRecipes
            0x74 => TagsWithEntities
        }
    }
    login Login {
        serverbound Serverbound {
            0x00 => LoginStart
            0x01 => EncryptionResponse
            0x02 => LoginPluginResponse
            // 0x03 => login_acknowledged (no internal equivalent)
        }
        clientbound Clientbound {
            0x00 => LoginDisconnect
            0x01 => EncryptionRequest
            0x02 => LoginSuccess_UUID_Properties
            0x03 => SetInitialCompression
            0x04 => LoginPluginRequest
        }
    }
    status Status {
        serverbound Serverbound {
            0x00 => StatusRequest
            0x01 => StatusPing
        }
        clientbound Clientbound {
            0x00 => StatusResponse
            0x01 => StatusPong
        }
    }
);
