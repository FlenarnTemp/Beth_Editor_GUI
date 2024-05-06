use std::{cell::RefCell, rc::Rc};

use crate::{check_loaded_data, data::{core::field::FieldData, self}, GLOBAL_PLUGIN};
use gtk::{
    glib::Value, prelude::*, ApplicationWindow, CellRendererText, ScrolledWindow, TreeStore,
    TreeView, TreeViewColumn,
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

    let right_scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .min_content_height(400)
        .min_content_width(200)
        .build();

    let right_grid = gtk::Grid::builder().build();
    right_scrolled_window.add(&right_grid);

    let treeview = build_treeview();

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .min_content_height(400)
        .min_content_width(200)
        .child(&treeview)
        .build();

    main_box.pack_start(&scrolled_window, false, true, 0);

    let object_window_table = Rc::new(RefCell::new(TreeView::new()));

    treeview.connect_cursor_changed(move |tree_view| {
        let selection = tree_view.selection();
        if check_loaded_data() {
            if let Some(_tree_model) = tree_view.model() {
                if let Some((tree_model, tree_iter)) = selection.selected() {
                    if let Ok(value) = tree_model.value(&tree_iter, 0).get::<String>() {
                        let mut data_id = 0;

                        right_grid.foreach(|child| {
                            right_grid.remove(child);
                        });

                        match value.as_str() {
                            "VoiceType" => {
                                data_id = 73;
                            }

                            "Color Form" => {
                                data_id = 101;
                            }

                            "ImpactDataSet" => {
                                data_id = 76;
                            }

                            _ => {
                                println!("Clicked: {:?}", value);
                            }
                        }

                        if data_id > 0 {
                            let new_object_window_table = create_sortable_table(data_id);
                            new_object_window_table.set_vexpand(true);
                            new_object_window_table.set_hexpand(true);
                            right_grid.add(&new_object_window_table);
                            right_grid.show_all();

                            *object_window_table.borrow_mut() = new_object_window_table;
                        }
                    } else {
                        println!("Value is not a valid String.");
                    }
                }
            }
        } else {
            println!("No data loaded.");
        }
    });

    main_box.pack_start(&right_scrolled_window, true, true, 0);
    window.set_child(Some(&main_box));
    window.show_all();
    window.present();
}

fn build_treeview() -> TreeView {
    let treeview = TreeView::builder().width_request(200).build();

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
    let actor_item_names = [
        "Actors", "Actor Action", "BodyPartData", "LeveledCharacter", "Perk", "TalkingActivator"
    ];
    
    for &item_name in &actor_item_names {
        let sub_actor_iter = treestore.append(Some(&actor_iter));
        treestore.set_value(&sub_actor_iter, 0, &item_name.to_value());
    }    

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
    let audio_item_names = [
        "Acoustic Space", "Animation Tag Set", "Category Snapshot", "Effect Chain", 
        "Music Track", "Music Type", "Reverb Parameters", "Sound Category", 
        "Sound Descriptor", "Sound Marker", "Sound Output Model"
    ];
    
    for &item_name in &audio_item_names {
        let sub_audio_iter = treestore.append(Some(&audio_iter));
        treestore.set_value(&sub_audio_iter, 0, &item_name.to_value());
    }    

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
    let character_item_names = [
        "Association Type", "Class", "Equip Slot", "Faction", "HeadPart", "MovementType",
        "Package", "Quest", "Race", "Relationship", "SM Event Node", "VoiceType"
    ];
    
    for &item_name in &character_item_names {
        let sub_character_iter = treestore.append(Some(&character_iter));
        treestore.set_value(&sub_character_iter, 0, &item_name.to_value());
    }    

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
    let item_names = [
        "Ammo", "Armor", "ArmorAddon", "Books", "Constructible Object", "Holotape", 
        "Ingredient", "Key", "LeveledItem", "MiscItem", "Object Mod", "Outfit", "Weapon"
    ];

    for &item_name in &item_names {
        let sub_items_iter = treestore.append(Some(&items_iter));
        treestore.set_value(&sub_items_iter, 0, &item_name.to_value());
    }

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
    let magic_item_names = [
        "Dual Cast Data", "Enchantment", "LeveledSpell", "Magic Effect", "Potion", "Spell"
    ];

    for &item_name in &magic_item_names {
        let sub_magic_iter = treestore.append(Some(&magic_iter));
        treestore.set_value(&sub_magic_iter, 0, &item_name.to_value());
    }

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

    /* Sub-levels */
    let miscellaneous_item_names = [
        "Actor Value", "AimModel", "AnimObject", "Art Object", "AttractionRule", "BendableSpline",
        "Collision Layer", "Color Form", "CombatStyle", "Component", "Damage Type", "DefaultObject",
        "FormList", "Global", "IdleMarker", "Instance Naming Rules", "Keyword", "LandTexture",
        "Layer", "LoadScreen", "Material Object", "Material Swap", "Message", "PackIn",
        "TextureSet", "Transform", "ZoomData"
    ];

    for &item_name in &miscellaneous_item_names {
        let sub_miscellaneous_iter = treestore.append(Some(&miscellaneous_iter));
        treestore.set_value(&sub_miscellaneous_iter, 0, &item_name.to_value());
    }

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
    let specialeffect_item_names = [
        "AddOnNode", "CameraShot", "Debris", "EffectShader", "Explosion", "Footstep",
        "Footstep Set", "GodRays", "Hazard", "Imagespace", "Imagespace Modifier",
        "ImpactData", "ImpactDataSet", "LensFlare", "Material Type", "Projectile"
    ];
    
    for &item_name in &specialeffect_item_names {
        let sub_specialeffect_iter = treestore.append(Some(&specialeffect_iter));
        treestore.set_value(&sub_specialeffect_iter, 0, &item_name.to_value());
    }

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
    let worlddata_item_names = [
        "Climate", "Encounter Zone", "Lighting Template", "Location", "Location Ref Type",
        "Shader Particle", "Visual Effect", "WaterType", "Weather"
    ];
    
    for &item_name in &worlddata_item_names {
        let sub_worlddata_iter = treestore.append(Some(&worlddata_iter));
        treestore.set_value(&sub_worlddata_iter, 0, &item_name.to_value());
    }    

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
    let worldobjects_item_names = [
        "Activator", "Container", "Door", "Flora", "Grass", "Light", 
        "MoveableStatic", "Static", "Terminal", "Tree"
    ];
    
    for &item_name in &worldobjects_item_names {
        let sub_worldobjects_iter = treestore.append(Some(&worldobjects_iter));
        treestore.set_value(&sub_worldobjects_iter, 0, &item_name.to_value());
    }

    /* Fill from all categories */
    let all_iter = treestore.append(None);
    treestore.set_value(&all_iter, 0, &"All".to_value());

    // Default to collapsed state.
    treeview.collapse_all();

    return treeview;
}

fn create_sortable_table(group_id: u32) -> TreeView {
    let treeview = TreeView::new();
    let treestore = TreeStore::new(&[String::static_type(), String::static_type()]);
    let edid_column = TreeViewColumn::new();
    let formid_column = TreeViewColumn::new();

    let cell_renderer = CellRendererText::new();

    treestore.set_sort_column_id(gtk::SortColumn::Index(0), gtk::SortType::Ascending);
    treeview.set_model(Some(&treestore));
    treeview.set_headers_visible(true);

    edid_column.set_title("Editor ID");
    edid_column.set_resizable(true);
    gtk::prelude::CellLayoutExt::pack_start(&edid_column, &cell_renderer, true);
    gtk::prelude::CellLayoutExt::add_attribute(&edid_column, &cell_renderer, "text", 0);
    treeview.append_column(&edid_column);

    formid_column.set_title("FormID");
    formid_column.set_resizable(true);
    gtk::prelude::CellLayoutExt::pack_start(&formid_column, &cell_renderer, true);
    gtk::prelude::CellLayoutExt::add_attribute(&formid_column, &cell_renderer, "text", 1);
    treeview.append_column(&formid_column);

    let plugin_mutex = GLOBAL_PLUGIN.lock().unwrap();

    if let Some(plugin) = &*plugin_mutex {
        let group_records = &plugin.groups[group_id as usize].records;

        for record in group_records {
            let row = treestore.append(None);
            if let Some(field_data) = &record.fields[0].data {
                let edid_value = match field_data {
                    FieldData::StringData(s) => Value::from(&s.trim_end_matches('\0')),
                    _ => {
                        // Handle other variants of FieldData here if needed
                        Value::from(&String::from("Default Value"))
                    }
                };
                treestore.set_value(&row, 0, &edid_value);
                treestore.set_value(&row, 1, &record.id.to_value());
                println!("{:?}", record.fields[0].data)
            }
        }

        gtk::main_iteration();
    }
    treeview
}