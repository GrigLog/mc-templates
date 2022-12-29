package griglog.my_modid.fabric

import MyModid
import net.fabricmc.api.ModInitializer

object MyModid : ModInitializer {
    override fun onInitialize() {
        System.out.println("FABRIC INIT")
        MyModid.init()
    }
}
