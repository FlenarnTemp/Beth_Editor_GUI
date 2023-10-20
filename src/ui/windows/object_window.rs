use gtk::{
    prelude::*, ApplicationWindow, CellRendererText, ScrolledWindow, TreeStore, TreeView,
    TreeViewColumn, TextView, TextBuffer, TextTag, TextTagTable,
};

pub fn build_ui() {
    let window = ApplicationWindow::builder()
        .default_height(600)
        .default_width(450)
        .title("Object Window")
        .build();

    window.set_deletable(false);
    window.set_type_hint(gtk::gdk::WindowTypeHint::Menu);
    window.set_keep_above(false);

    let main_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(1)
        .build();

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .min_content_height(400)
        .min_content_width(200)
        .child(&build_treeview())
        .build();

    let text_view = TextView::builder().build();

    let text_tag_table = TextTagTable::new();
    let text_buffer = TextBuffer::new(Some(&text_tag_table));
    text_view.set_buffer(Some(&text_buffer));

    main_box.pack_start(&scrolled_window, false, true, 0);
    main_box.pack_start(&text_view, true, true, 0);

    window.set_child(Some(&main_box));
    window.show_all();
    window.present();
}

fn build_treeview() -> TreeView {
    let treeview = TreeView::builder()
    .width_request(200)
    
    .build();

    let treestore = TreeStore::new(&[String::static_type()]);
    treeview.set_model(Some(&treestore));
    treeview.set_headers_visible(false);

    // Create a column and associate it with the TreeStore
    let column = TreeViewColumn::builder().build();
    let cell_renderer = CellRendererText::builder().build();
    gtk::prelude::CellLayoutExt::pack_start(&column, &cell_renderer, true);
    gtk::prelude::TreeViewColumnExt::add_attribute(&column, &cell_renderer, "text", 0);
    treeview.append_column(&column);

    /*
     * Actor Structure
     * Actors ->
     *      Actors -> Record Filters
     *      Actor Action
     *      BodyPartData -> Record Filters
     *      LeveledCharacter -> Record Filters
     *      Perk
     *      TalkingActivator -> Record Filters
     */
    let actor_iter = treestore.append(None);
    treestore.set_value(&actor_iter, 0, &"Actors".to_value());

    /* Sub-levels */

    let sub_actor_iter1 = treestore.append(Some(&actor_iter));
    treestore.set_value(&sub_actor_iter1, 0, &"Actors".to_value());

    let sub_actor_iter2 = treestore.append(Some(&actor_iter));
    treestore.set_value(&sub_actor_iter2, 0, &"Actor Action".to_value());

    let sub_actor_iter3 = treestore.append(Some(&actor_iter));
    treestore.set_value(&sub_actor_iter3, 0, &"BodyPartData".to_value());

    let sub_actor_iter4 = treestore.append(Some(&actor_iter));
    treestore.set_value(&sub_actor_iter4, 0, &"LeveledCharacter".to_value());

    let sub_actor_iter5 = treestore.append(Some(&actor_iter));
    treestore.set_value(&sub_actor_iter5, 0, &"Perk".to_value());

    let sub_actor_iter6 = treestore.append(Some(&actor_iter));
    treestore.set_value(&sub_actor_iter6, 0, &"TalkingActivator".to_value());
    /*==============================================================================*/

    /*
     * Audio Structure
     * Audio ->
     *      Acoustic Space
     *      Animation Tag Set
     *      Category Snapshot
     *      Effect Chain
     *      Music Track
     *      Music Type
     *      Reverb Parameters
     *      Sound Category
     *      Sound Descriptor -> Record Filters
     *      Sound Keyword Mapping
     *      Sound Marker
     *      Sound Output Model
     */
    let audio_iter = treestore.append(None);
    treestore.set_value(&audio_iter, 0, &"Audio".to_value());

    /* Sub-levels */

    let sub_audio_iter1 = treestore.append(Some(&audio_iter));
    treestore.set_value(&sub_audio_iter1, 0, &"Acoustic Space".to_value());

    let sub_audio_iter2 = treestore.append(Some(&audio_iter));
    treestore.set_value(&sub_audio_iter2, 0, &"Animation Tag Set".to_value());

    let sub_audio_iter3 = treestore.append(Some(&audio_iter));
    treestore.set_value(&sub_audio_iter3, 0, &"Category Snapshot".to_value());

    let sub_audio_iter4 = treestore.append(Some(&audio_iter));
    treestore.set_value(&sub_audio_iter4, 0, &"Effect Chain".to_value());

    let sub_audio_iter5 = treestore.append(Some(&audio_iter));
    treestore.set_value(&sub_audio_iter5, 0, &"Music Track".to_value());

    let sub_audio_iter6 = treestore.append(Some(&audio_iter));
    treestore.set_value(&sub_audio_iter6, 0, &"Music Type".to_value());

    let sub_audio_iter7 = treestore.append(Some(&audio_iter));
    treestore.set_value(&sub_audio_iter7, 0, &"Reverb Parameters".to_value());

    let sub_audio_iter8 = treestore.append(Some(&audio_iter));
    treestore.set_value(&sub_audio_iter8, 0, &"Sound Category".to_value());

    let sub_audio_iter9 = treestore.append(Some(&audio_iter));
    treestore.set_value(&sub_audio_iter9, 0, &"Sound Descriptor".to_value());

    let sub_audio_iter10 = treestore.append(Some(&audio_iter));
    treestore.set_value(&sub_audio_iter10, 0, &"Sound Marker".to_value());

    let sub_audio_iter11 = treestore.append(Some(&audio_iter));
    treestore.set_value(&sub_audio_iter11, 0, &"Sound Output Model".to_value());
    /*==============================================================================*/

    /*
     * Character Structure
     * Character ->
     *      Association Type
     *      Class
     *      Equip Slot
     *      Faction
     *      HeadPart -> Record Filters
     *      MovementType
     *      Package -> Record Filters
     *      Quest -> Record Filters
     *      Race
     *      Relationship
     *      SM Event Node
     *      VoiceType
     */
    let character_iter = treestore.append(None);
    treestore.set_value(&character_iter, 0, &"Character".to_value());

    /* Sub-levels */

    let sub_character_iter1 = treestore.append(Some(&character_iter));
    treestore.set_value(&sub_character_iter1, 0, &"Association Type".to_value());

    let sub_character_iter2 = treestore.append(Some(&character_iter));
    treestore.set_value(&sub_character_iter2, 0, &"Class".to_value());

    let sub_character_iter3 = treestore.append(Some(&character_iter));
    treestore.set_value(&sub_character_iter3, 0, &"Equip Slot".to_value());

    let sub_character_iter4 = treestore.append(Some(&character_iter));
    treestore.set_value(&sub_character_iter4, 0, &"Faction".to_value());

    let sub_character_iter5 = treestore.append(Some(&character_iter));
    treestore.set_value(&sub_character_iter5, 0, &"HeadPart".to_value());

    let sub_character_iter6 = treestore.append(Some(&character_iter));
    treestore.set_value(&sub_character_iter6, 0, &"MovementType".to_value());

    let sub_character_iter7 = treestore.append(Some(&character_iter));
    treestore.set_value(&sub_character_iter7, 0, &"Package".to_value());

    let sub_character_iter8 = treestore.append(Some(&character_iter));
    treestore.set_value(&sub_character_iter8, 0, &"Quest".to_value());

    let sub_character_iter9 = treestore.append(Some(&character_iter));
    treestore.set_value(&sub_character_iter9, 0, &"Race".to_value());

    let sub_character_iter10 = treestore.append(Some(&character_iter));
    treestore.set_value(&sub_character_iter10, 0, &"Relationship".to_value());

    let sub_character_iter11 = treestore.append(Some(&character_iter));
    treestore.set_value(&sub_character_iter11, 0, &"SM Event Node".to_value());

    let sub_character_iter12 = treestore.append(Some(&character_iter));
    treestore.set_value(&sub_character_iter12, 0, &"VoiceType".to_value());
    /*==============================================================================*/

    /*
     * Items Structure
     * Items ->
     *      Ammo -> Record Filters
     *      Armor -> Record Filters
     *      ArmorAddon
     *      Books -> Record Filters
     *      Constructible Object
     *      Holotape -> Record Filters
     *      Ingredient -> Record Filters
     *      Key -> Record Filters
     *      LeveledItem
     *      MiscItem -> Record Filters
     *      Object Mod -> Record Filters
     *      Outfit
     *      Weapon -> Record Filters
     */
    let items_iter = treestore.append(None);
    treestore.set_value(&items_iter, 0, &"Items".to_value());

    /* Sub-levels */

    let sub_items_iter1 = treestore.append(Some(&items_iter));
    treestore.set_value(&sub_items_iter1, 0, &"Ammo".to_value());

    let sub_items_iter2 = treestore.append(Some(&items_iter));
    treestore.set_value(&sub_items_iter2, 0, &"Armor".to_value());

    let sub_items_iter3 = treestore.append(Some(&items_iter));
    treestore.set_value(&sub_items_iter3, 0, &"ArmorAddon".to_value());

    let sub_items_iter4 = treestore.append(Some(&items_iter));
    treestore.set_value(&sub_items_iter4, 0, &"Books".to_value());

    let sub_items_iter5 = treestore.append(Some(&items_iter));
    treestore.set_value(&sub_items_iter5, 0, &"Constructible Object".to_value());

    let sub_items_iter6 = treestore.append(Some(&items_iter));
    treestore.set_value(&sub_items_iter6, 0, &"Holotape".to_value());

    let sub_items_iter7 = treestore.append(Some(&items_iter));
    treestore.set_value(&sub_items_iter7, 0, &"Ingredient".to_value());

    let sub_items_iter8 = treestore.append(Some(&items_iter));
    treestore.set_value(&sub_items_iter8, 0, &"Key".to_value());

    let sub_items_iter9 = treestore.append(Some(&items_iter));
    treestore.set_value(&sub_items_iter9, 0, &"LeveledItem".to_value());

    let sub_items_iter10 = treestore.append(Some(&items_iter));
    treestore.set_value(&sub_items_iter10, 0, &"MiscItem".to_value());

    let sub_items_iter11 = treestore.append(Some(&items_iter));
    treestore.set_value(&sub_items_iter11, 0, &"Object Mod".to_value());

    let sub_items_iter12 = treestore.append(Some(&items_iter));
    treestore.set_value(&sub_items_iter12, 0, &"Outfit".to_value());

    let sub_items_iter13 = treestore.append(Some(&items_iter));
    treestore.set_value(&sub_items_iter13, 0, &"Weapon".to_value());
    /*==============================================================================*/

    /*
     * Magic Structure
     * Magic ->
     *      Dual Cast Data
     *      Enchantment
     *      LeveledSpell
     *      MagicEffect
     *      Potion -> Record Filters
     *      Spell -> Record Filters
     */
    let magic_iter = treestore.append(None);
    treestore.set_value(&magic_iter, 0, &"Magic".to_value());

    /* Sub-levels */

    let sub_magic_iter1 = treestore.append(Some(&magic_iter));
    treestore.set_value(&sub_magic_iter1, 0, &"Dual Cast Data".to_value());

    let sub_magic_iter = treestore.append(Some(&magic_iter));
    treestore.set_value(&sub_magic_iter, 0, &"Enchantment".to_value());

    let sub_magic_iter3 = treestore.append(Some(&magic_iter));
    treestore.set_value(&sub_magic_iter3, 0, &"LeveledSpell".to_value());

    let sub_magic_iter4 = treestore.append(Some(&magic_iter));
    treestore.set_value(&sub_magic_iter4, 0, &"Magic Effect".to_value());

    let sub_magic_iter5 = treestore.append(Some(&magic_iter));
    treestore.set_value(&sub_magic_iter5, 0, &"Potion".to_value());

    let sub_magic_iter6 = treestore.append(Some(&magic_iter));
    treestore.set_value(&sub_magic_iter6, 0, &"Spell".to_value());
    /*==============================================================================*/

     /*
     * Miscellaneous Structure
     * Miscellaneous ->
     *      Actor Value
     *      AimModel
     *      AnimObject -> Record Filters
     *      Art Object -> Record Filters
     *      AttractionRule
     *      BendableSpline 
     *      Collision Layer
     *      ColorForm
     *      CombatStyle
     *      Component
     *      Damage Type
     *      DefaultObject
     *      FormList
     *      Global
     *      IdleMarker -> Record Filters
     *      Instance Naming Rules
     *      Keyword -> Record Filters
     *      LandTexture
     *      Layer
     *      LoadScreen
     *      Material Object -> Record Filters
     *      Material Swap -> Record Filters
     *      Message
     *      PackIn -> Record Filters
     *      TextureSet
     *      Transform
     *      ZoomData
     */

    let miscellaneous_iter = treestore.append(None);
    treestore.set_value(&miscellaneous_iter, 0, &"Miscellaneous".to_value());

    let sub_miscellaneous_iter1 = treestore.append(Some(&miscellaneous_iter));
    treestore.set_value(&sub_miscellaneous_iter1, 0, &"Actor Value".to_value());

    let sub_miscellaneous_iter2 = treestore.append(Some(&miscellaneous_iter));
    treestore.set_value(&sub_miscellaneous_iter2, 0, &"AimModel".to_value());

    let sub_miscellaneous_iter3 = treestore.append(Some(&miscellaneous_iter));
    treestore.set_value(&sub_miscellaneous_iter3, 0, &"AnimObject".to_value());

    let sub_miscellaneous_iter4 = treestore.append(Some(&miscellaneous_iter));
    treestore.set_value(&sub_miscellaneous_iter4, 0, &"Art Object".to_value());

    let sub_miscellaneous_iter5 = treestore.append(Some(&miscellaneous_iter));
    treestore.set_value(&sub_miscellaneous_iter5, 0, &"AttractionRule".to_value());

    let sub_miscellaneous_iter6 = treestore.append(Some(&miscellaneous_iter));
    treestore.set_value(&sub_miscellaneous_iter6, 0, &"BendableSpline".to_value());

    let sub_miscellaneous_iter7 = treestore.append(Some(&miscellaneous_iter));
    treestore.set_value(&sub_miscellaneous_iter7, 0, &"Collision Layer".to_value());

    let sub_miscellaneous_iter8 = treestore.append(Some(&miscellaneous_iter));
    treestore.set_value(&sub_miscellaneous_iter8, 0, &"Color Form".to_value());

    let sub_miscellaneous_iter9 = treestore.append(Some(&miscellaneous_iter));
    treestore.set_value(&sub_miscellaneous_iter9, 0, &"CombatStyle".to_value());

    let sub_miscellaneous_iter10 = treestore.append(Some(&miscellaneous_iter));
    treestore.set_value(&sub_miscellaneous_iter10, 0, &"Component".to_value());

    let sub_miscellaneous_iter11 = treestore.append(Some(&miscellaneous_iter));
    treestore.set_value(&sub_miscellaneous_iter11, 0, &"Damage Type".to_value());

    let sub_miscellaneous_iter12 = treestore.append(Some(&miscellaneous_iter));
    treestore.set_value(&sub_miscellaneous_iter12, 0, &"DefaultObject".to_value());

    let sub_miscellaneous_iter13 = treestore.append(Some(&miscellaneous_iter));
    treestore.set_value(&sub_miscellaneous_iter13, 0, &"FormList".to_value());

    let sub_miscellaneous_iter14 = treestore.append(Some(&miscellaneous_iter));
    treestore.set_value(&sub_miscellaneous_iter14, 0, &"Global".to_value());

    let sub_miscellaneous_iter15 = treestore.append(Some(&miscellaneous_iter));
    treestore.set_value(&sub_miscellaneous_iter15, 0, &"IdleMarker".to_value());

    let sub_miscellaneous_iter16 = treestore.append(Some(&miscellaneous_iter));
    treestore.set_value(&sub_miscellaneous_iter16, 0, &"Instance Naming Rules".to_value());

    let sub_miscellaneous_iter17 = treestore.append(Some(&miscellaneous_iter));
    treestore.set_value(&sub_miscellaneous_iter17, 0, &"Keyword".to_value());

    let sub_miscellaneous_iter18 = treestore.append(Some(&miscellaneous_iter));
    treestore.set_value(&sub_miscellaneous_iter18, 0, &"LandTexture".to_value());

    let sub_miscellaneous_iter19 = treestore.append(Some(&miscellaneous_iter));
    treestore.set_value(&sub_miscellaneous_iter19, 0, &"Layer".to_value());

    let sub_miscellaneous_iter20 = treestore.append(Some(&miscellaneous_iter));
    treestore.set_value(&sub_miscellaneous_iter20, 0, &"LoadScreen".to_value());

    let sub_miscellaneous_iter21 = treestore.append(Some(&miscellaneous_iter));
    treestore.set_value(&sub_miscellaneous_iter21, 0, &"Material Object".to_value());

    let sub_miscellaneous_iter22 = treestore.append(Some(&miscellaneous_iter));
    treestore.set_value(&sub_miscellaneous_iter22, 0, &"Material Swap".to_value());

    let sub_miscellaneous_iter23 = treestore.append(Some(&miscellaneous_iter));
    treestore.set_value(&sub_miscellaneous_iter23, 0, &"Message".to_value());

    let sub_miscellaneous_iter24 = treestore.append(Some(&miscellaneous_iter));
    treestore.set_value(&sub_miscellaneous_iter24, 0, &"PackIn".to_value());

    let sub_miscellaneous_iter25 = treestore.append(Some(&miscellaneous_iter));
    treestore.set_value(&sub_miscellaneous_iter25, 0, &"TextureSet".to_value());

    let sub_miscellaneous_iter26 = treestore.append(Some(&miscellaneous_iter));
    treestore.set_value(&sub_miscellaneous_iter26, 0, &"Transform".to_value());

    let sub_miscellaneous_iter27 = treestore.append(Some(&miscellaneous_iter));
    treestore.set_value(&sub_miscellaneous_iter27, 0, &"ZoomData".to_value());
    /*==============================================================================*/

    /*
     * SpecialEffect Structure
     * SpecialEffect ->
     *      AddOnNode -> Record Filters
     *      CameraShot -> Record Filters
     *      Debris
     *      EffectShader
     *      Explosion
     *      Footstep
     *      Footstep Set
     *      GodRays
     *      Hazard
     *      Imagespace
     *      Imagespace Modifier
     *      ImpactData -> Record Filters
     *      ImpactDataSet
     *      LensFlare
     *      Material Type -> Record Filters
     *      Projectile
     */
    let specialeffect_iter = treestore.append(None);
    treestore.set_value(&specialeffect_iter, 0, &"SpecialEffect".to_value());

    /* Sub-levels */

    let sub_specialeffect_iter1 = treestore.append(Some(&specialeffect_iter));
    treestore.set_value(&sub_specialeffect_iter1, 0, &"AddOnNode".to_value());

    let sub_specialeffect_iter1 = treestore.append(Some(&specialeffect_iter));
    treestore.set_value(&sub_specialeffect_iter1, 0, &"CameraShot".to_value());

    let sub_specialeffect_iter1 = treestore.append(Some(&specialeffect_iter));
    treestore.set_value(&sub_specialeffect_iter1, 0, &"Debris".to_value());

    let sub_specialeffect_iter1 = treestore.append(Some(&specialeffect_iter));
    treestore.set_value(&sub_specialeffect_iter1, 0, &"EffectShader".to_value());

    let sub_specialeffect_iter1 = treestore.append(Some(&specialeffect_iter));
    treestore.set_value(&sub_specialeffect_iter1, 0, &"Explosion".to_value());
    
    let sub_specialeffect_iter1 = treestore.append(Some(&specialeffect_iter));
    treestore.set_value(&sub_specialeffect_iter1, 0, &"Footstep".to_value());

    let sub_specialeffect_iter1 = treestore.append(Some(&specialeffect_iter));
    treestore.set_value(&sub_specialeffect_iter1, 0, &"Footstep Set".to_value());

    let sub_specialeffect_iter1 = treestore.append(Some(&specialeffect_iter));
    treestore.set_value(&sub_specialeffect_iter1, 0, &"GodRays".to_value());
    
    let sub_specialeffect_iter1 = treestore.append(Some(&specialeffect_iter));
    treestore.set_value(&sub_specialeffect_iter1, 0, &"Hazard".to_value());

    let sub_specialeffect_iter1 = treestore.append(Some(&specialeffect_iter));
    treestore.set_value(&sub_specialeffect_iter1, 0, &"Imagespace".to_value());

    let sub_specialeffect_iter1 = treestore.append(Some(&specialeffect_iter));
    treestore.set_value(&sub_specialeffect_iter1, 0, &"Imagespace Modifier".to_value());

    let sub_specialeffect_iter1 = treestore.append(Some(&specialeffect_iter));
    treestore.set_value(&sub_specialeffect_iter1, 0, &"ImpactData".to_value());

    let sub_specialeffect_iter1 = treestore.append(Some(&specialeffect_iter));
    treestore.set_value(&sub_specialeffect_iter1, 0, &"ImpactDataSet".to_value());

    let sub_specialeffect_iter1 = treestore.append(Some(&specialeffect_iter));
    treestore.set_value(&sub_specialeffect_iter1, 0, &"LensFlare".to_value());

    let sub_specialeffect_iter1 = treestore.append(Some(&specialeffect_iter));
    treestore.set_value(&sub_specialeffect_iter1, 0, &"Material Type".to_value());

    let sub_specialeffect_iter1 = treestore.append(Some(&specialeffect_iter));
    treestore.set_value(&sub_specialeffect_iter1, 0, &"Projectile".to_value());
    /*==============================================================================*/

    /*
     * WorldData Structure
     * WorldData ->
     *      Climate
     *      Encounter Zone
     *      Lighting Template
     *      Location -> Record Filters
     *      Location Ref Type
     *      Shader Particle
     *      Visual Effect
     *      WaterType
     *      Weather
     */

    let worlddata_iter = treestore.append(None);
    treestore.set_value(&worlddata_iter, 0, &"WorldData".to_value());

    /* Sub-levels */

    let sub_worlddata_iter1 = treestore.append(Some(&worlddata_iter));
    treestore.set_value(&sub_worlddata_iter1, 0, &"Climate".to_value());

    let sub_worlddata_iter2 = treestore.append(Some(&worlddata_iter));
    treestore.set_value(&sub_worlddata_iter2, 0, &"Encounter Zone".to_value());

    let sub_worlddata_iter3 = treestore.append(Some(&worlddata_iter));
    treestore.set_value(&sub_worlddata_iter3, 0, &"Lighting Template".to_value());

    let sub_worlddata_iter4 = treestore.append(Some(&worlddata_iter));
    treestore.set_value(&sub_worlddata_iter4, 0, &"Location".to_value());

    let sub_worlddata_iter5 = treestore.append(Some(&worlddata_iter));
    treestore.set_value(&sub_worlddata_iter5, 0, &"Location Ref Type".to_value());

    let sub_worlddata_iter6 = treestore.append(Some(&worlddata_iter));
    treestore.set_value(&sub_worlddata_iter6, 0, &"Shader Particle".to_value());
    
    let sub_worlddata_iter7 = treestore.append(Some(&worlddata_iter));
    treestore.set_value(&sub_worlddata_iter7, 0, &"Visual Effect".to_value());

    let sub_worlddata_iter8 = treestore.append(Some(&worlddata_iter));
    treestore.set_value(&sub_worlddata_iter8, 0, &"WaterType".to_value());

    let sub_worlddata_iter9 = treestore.append(Some(&worlddata_iter));
    treestore.set_value(&sub_worlddata_iter9, 0, &"Weather".to_value());
    /*==============================================================================*/

    /*
     * WorldObjects Structure
     * WorldObjects ->
     *      Activator -> Record Filters
     *      Container -> Record Filters
     *      Door -> Record Filters
     *      Flora -> Record Filters
     *      Grass -> Record Filters
     *      Light -> Record Filters
     *      MovableStatic -> Record Filters
     *      Static -> Record Filters
     *      Terminal -> Record Filters
     *      Tree -> Record Filters
     */
    let worldobjects_iter = treestore.append(None);
    treestore.set_value(&worldobjects_iter, 0, &"WorldObjects".to_value());

    /* Sub-levels */

    let sub_worldobjects_iter1 = treestore.append(Some(&worldobjects_iter));
    treestore.set_value(&sub_worldobjects_iter1, 0, &"Activator".to_value());

    let sub_worldobjects_iter2 = treestore.append(Some(&worldobjects_iter));
    treestore.set_value(&sub_worldobjects_iter2, 0, &"Container".to_value());

    let sub_worldobjects_iter3 = treestore.append(Some(&worldobjects_iter));
    treestore.set_value(&sub_worldobjects_iter3, 0, &"Door".to_value());

    let sub_worldobjects_iter4 = treestore.append(Some(&worldobjects_iter));
    treestore.set_value(&sub_worldobjects_iter4, 0, &"Flora".to_value());

    let sub_worldobjects_iter5 = treestore.append(Some(&worldobjects_iter));
    treestore.set_value(&sub_worldobjects_iter5, 0, &"Grass".to_value());

    let sub_worldobjects_iter6 = treestore.append(Some(&worldobjects_iter));
    treestore.set_value(&sub_worldobjects_iter6, 0, &"Light".to_value());

    let sub_worldobjects_iter7 = treestore.append(Some(&worldobjects_iter));
    treestore.set_value(&sub_worldobjects_iter7, 0, &"MoveableStatic".to_value());

    let sub_worldobjects_iter8 = treestore.append(Some(&worldobjects_iter));
    treestore.set_value(&sub_worldobjects_iter8, 0, &"Static".to_value());

    let sub_worldobjects_iter9 = treestore.append(Some(&worldobjects_iter));
    treestore.set_value(&sub_worldobjects_iter9, 0, &"Terminal".to_value());

    let sub_worldobjects_iter10 = treestore.append(Some(&worldobjects_iter));
    treestore.set_value(&sub_worldobjects_iter10, 0, &"Tree".to_value());
    /*==============================================================================*/

    let all_iter = treestore.append(None);
    treestore.set_value(&all_iter, 0, &"All".to_value());
    /*==============================================================================*/

    // Default to collapsed state.
    treeview.collapse_all();

    treeview.connect_cursor_changed(|tree_view| {
        if let selection = tree_view.selection() {
            if let Some(tree_model) = tree_view.model() {
                if let Some((tree_model, tree_iter)) = selection.selected() {
                    if let value = tree_model.value(&tree_iter, 0).get::<String>() {
                        println!("Clicked: {:?}", value);
                    }
                }
            }
        }
    });

    return treeview;
}
