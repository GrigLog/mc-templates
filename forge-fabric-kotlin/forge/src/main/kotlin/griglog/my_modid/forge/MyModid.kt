package griglog.my_modid.forge

import MyModid
import dev.architectury.platform.forge.EventBuses
import net.minecraftforge.fml.common.Mod
import thedarkcolour.kotlinforforge.forge.MOD_CONTEXT

@Mod(MyModid.id)
object MyModid {
    init {
        System.out.println("FORGE INIT")
        EventBuses.registerModEventBus(MyModid.id, MOD_CONTEXT.getKEventBus())
        MyModid.init()
    }
}
