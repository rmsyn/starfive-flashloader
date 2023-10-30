use super::FlashSector;

pub(crate) const SECTORS_LEN: usize = 0x400;

pub(crate) const SECTORS: [FlashSector; SECTORS_LEN] = [
    FlashSector::create(0x21000000),
    FlashSector::create(0x21001000),
    FlashSector::create(0x21002000),
    FlashSector::create(0x21003000),
    FlashSector::create(0x21004000),
    FlashSector::create(0x21005000),
    FlashSector::create(0x21006000),
    FlashSector::create(0x21007000),
    FlashSector::create(0x21008000),
    FlashSector::create(0x21009000),
    FlashSector::create(0x2100a000),
    FlashSector::create(0x2100b000),
    FlashSector::create(0x2100c000),
    FlashSector::create(0x2100d000),
    FlashSector::create(0x2100e000),
    FlashSector::create(0x2100f000),
    FlashSector::create(0x21010000),
    FlashSector::create(0x21011000),
    FlashSector::create(0x21012000),
    FlashSector::create(0x21013000),
    FlashSector::create(0x21014000),
    FlashSector::create(0x21015000),
    FlashSector::create(0x21016000),
    FlashSector::create(0x21017000),
    FlashSector::create(0x21018000),
    FlashSector::create(0x21019000),
    FlashSector::create(0x2101a000),
    FlashSector::create(0x2101b000),
    FlashSector::create(0x2101c000),
    FlashSector::create(0x2101d000),
    FlashSector::create(0x2101e000),
    FlashSector::create(0x2101f000),
    FlashSector::create(0x21020000),
    FlashSector::create(0x21021000),
    FlashSector::create(0x21022000),
    FlashSector::create(0x21023000),
    FlashSector::create(0x21024000),
    FlashSector::create(0x21025000),
    FlashSector::create(0x21026000),
    FlashSector::create(0x21027000),
    FlashSector::create(0x21028000),
    FlashSector::create(0x21029000),
    FlashSector::create(0x2102a000),
    FlashSector::create(0x2102b000),
    FlashSector::create(0x2102c000),
    FlashSector::create(0x2102d000),
    FlashSector::create(0x2102e000),
    FlashSector::create(0x2102f000),
    FlashSector::create(0x21030000),
    FlashSector::create(0x21031000),
    FlashSector::create(0x21032000),
    FlashSector::create(0x21033000),
    FlashSector::create(0x21034000),
    FlashSector::create(0x21035000),
    FlashSector::create(0x21036000),
    FlashSector::create(0x21037000),
    FlashSector::create(0x21038000),
    FlashSector::create(0x21039000),
    FlashSector::create(0x2103a000),
    FlashSector::create(0x2103b000),
    FlashSector::create(0x2103c000),
    FlashSector::create(0x2103d000),
    FlashSector::create(0x2103e000),
    FlashSector::create(0x2103f000),
    FlashSector::create(0x21040000),
    FlashSector::create(0x21041000),
    FlashSector::create(0x21042000),
    FlashSector::create(0x21043000),
    FlashSector::create(0x21044000),
    FlashSector::create(0x21045000),
    FlashSector::create(0x21046000),
    FlashSector::create(0x21047000),
    FlashSector::create(0x21048000),
    FlashSector::create(0x21049000),
    FlashSector::create(0x2104a000),
    FlashSector::create(0x2104b000),
    FlashSector::create(0x2104c000),
    FlashSector::create(0x2104d000),
    FlashSector::create(0x2104e000),
    FlashSector::create(0x2104f000),
    FlashSector::create(0x21050000),
    FlashSector::create(0x21051000),
    FlashSector::create(0x21052000),
    FlashSector::create(0x21053000),
    FlashSector::create(0x21054000),
    FlashSector::create(0x21055000),
    FlashSector::create(0x21056000),
    FlashSector::create(0x21057000),
    FlashSector::create(0x21058000),
    FlashSector::create(0x21059000),
    FlashSector::create(0x2105a000),
    FlashSector::create(0x2105b000),
    FlashSector::create(0x2105c000),
    FlashSector::create(0x2105d000),
    FlashSector::create(0x2105e000),
    FlashSector::create(0x2105f000),
    FlashSector::create(0x21060000),
    FlashSector::create(0x21061000),
    FlashSector::create(0x21062000),
    FlashSector::create(0x21063000),
    FlashSector::create(0x21064000),
    FlashSector::create(0x21065000),
    FlashSector::create(0x21066000),
    FlashSector::create(0x21067000),
    FlashSector::create(0x21068000),
    FlashSector::create(0x21069000),
    FlashSector::create(0x2106a000),
    FlashSector::create(0x2106b000),
    FlashSector::create(0x2106c000),
    FlashSector::create(0x2106d000),
    FlashSector::create(0x2106e000),
    FlashSector::create(0x2106f000),
    FlashSector::create(0x21070000),
    FlashSector::create(0x21071000),
    FlashSector::create(0x21072000),
    FlashSector::create(0x21073000),
    FlashSector::create(0x21074000),
    FlashSector::create(0x21075000),
    FlashSector::create(0x21076000),
    FlashSector::create(0x21077000),
    FlashSector::create(0x21078000),
    FlashSector::create(0x21079000),
    FlashSector::create(0x2107a000),
    FlashSector::create(0x2107b000),
    FlashSector::create(0x2107c000),
    FlashSector::create(0x2107d000),
    FlashSector::create(0x2107e000),
    FlashSector::create(0x2107f000),
    FlashSector::create(0x21080000),
    FlashSector::create(0x21081000),
    FlashSector::create(0x21082000),
    FlashSector::create(0x21083000),
    FlashSector::create(0x21084000),
    FlashSector::create(0x21085000),
    FlashSector::create(0x21086000),
    FlashSector::create(0x21087000),
    FlashSector::create(0x21088000),
    FlashSector::create(0x21089000),
    FlashSector::create(0x2108a000),
    FlashSector::create(0x2108b000),
    FlashSector::create(0x2108c000),
    FlashSector::create(0x2108d000),
    FlashSector::create(0x2108e000),
    FlashSector::create(0x2108f000),
    FlashSector::create(0x21090000),
    FlashSector::create(0x21091000),
    FlashSector::create(0x21092000),
    FlashSector::create(0x21093000),
    FlashSector::create(0x21094000),
    FlashSector::create(0x21095000),
    FlashSector::create(0x21096000),
    FlashSector::create(0x21097000),
    FlashSector::create(0x21098000),
    FlashSector::create(0x21099000),
    FlashSector::create(0x2109a000),
    FlashSector::create(0x2109b000),
    FlashSector::create(0x2109c000),
    FlashSector::create(0x2109d000),
    FlashSector::create(0x2109e000),
    FlashSector::create(0x2109f000),
    FlashSector::create(0x210a0000),
    FlashSector::create(0x210a1000),
    FlashSector::create(0x210a2000),
    FlashSector::create(0x210a3000),
    FlashSector::create(0x210a4000),
    FlashSector::create(0x210a5000),
    FlashSector::create(0x210a6000),
    FlashSector::create(0x210a7000),
    FlashSector::create(0x210a8000),
    FlashSector::create(0x210a9000),
    FlashSector::create(0x210aa000),
    FlashSector::create(0x210ab000),
    FlashSector::create(0x210ac000),
    FlashSector::create(0x210ad000),
    FlashSector::create(0x210ae000),
    FlashSector::create(0x210af000),
    FlashSector::create(0x210b0000),
    FlashSector::create(0x210b1000),
    FlashSector::create(0x210b2000),
    FlashSector::create(0x210b3000),
    FlashSector::create(0x210b4000),
    FlashSector::create(0x210b5000),
    FlashSector::create(0x210b6000),
    FlashSector::create(0x210b7000),
    FlashSector::create(0x210b8000),
    FlashSector::create(0x210b9000),
    FlashSector::create(0x210ba000),
    FlashSector::create(0x210bb000),
    FlashSector::create(0x210bc000),
    FlashSector::create(0x210bd000),
    FlashSector::create(0x210be000),
    FlashSector::create(0x210bf000),
    FlashSector::create(0x210c0000),
    FlashSector::create(0x210c1000),
    FlashSector::create(0x210c2000),
    FlashSector::create(0x210c3000),
    FlashSector::create(0x210c4000),
    FlashSector::create(0x210c5000),
    FlashSector::create(0x210c6000),
    FlashSector::create(0x210c7000),
    FlashSector::create(0x210c8000),
    FlashSector::create(0x210c9000),
    FlashSector::create(0x210ca000),
    FlashSector::create(0x210cb000),
    FlashSector::create(0x210cc000),
    FlashSector::create(0x210cd000),
    FlashSector::create(0x210ce000),
    FlashSector::create(0x210cf000),
    FlashSector::create(0x210d0000),
    FlashSector::create(0x210d1000),
    FlashSector::create(0x210d2000),
    FlashSector::create(0x210d3000),
    FlashSector::create(0x210d4000),
    FlashSector::create(0x210d5000),
    FlashSector::create(0x210d6000),
    FlashSector::create(0x210d7000),
    FlashSector::create(0x210d8000),
    FlashSector::create(0x210d9000),
    FlashSector::create(0x210da000),
    FlashSector::create(0x210db000),
    FlashSector::create(0x210dc000),
    FlashSector::create(0x210dd000),
    FlashSector::create(0x210de000),
    FlashSector::create(0x210df000),
    FlashSector::create(0x210e0000),
    FlashSector::create(0x210e1000),
    FlashSector::create(0x210e2000),
    FlashSector::create(0x210e3000),
    FlashSector::create(0x210e4000),
    FlashSector::create(0x210e5000),
    FlashSector::create(0x210e6000),
    FlashSector::create(0x210e7000),
    FlashSector::create(0x210e8000),
    FlashSector::create(0x210e9000),
    FlashSector::create(0x210ea000),
    FlashSector::create(0x210eb000),
    FlashSector::create(0x210ec000),
    FlashSector::create(0x210ed000),
    FlashSector::create(0x210ee000),
    FlashSector::create(0x210ef000),
    FlashSector::create(0x210f0000),
    FlashSector::create(0x210f1000),
    FlashSector::create(0x210f2000),
    FlashSector::create(0x210f3000),
    FlashSector::create(0x210f4000),
    FlashSector::create(0x210f5000),
    FlashSector::create(0x210f6000),
    FlashSector::create(0x210f7000),
    FlashSector::create(0x210f8000),
    FlashSector::create(0x210f9000),
    FlashSector::create(0x210fa000),
    FlashSector::create(0x210fb000),
    FlashSector::create(0x210fc000),
    FlashSector::create(0x210fd000),
    FlashSector::create(0x210fe000),
    FlashSector::create(0x210ff000),
    FlashSector::create(0x21100000),
    FlashSector::create(0x21101000),
    FlashSector::create(0x21102000),
    FlashSector::create(0x21103000),
    FlashSector::create(0x21104000),
    FlashSector::create(0x21105000),
    FlashSector::create(0x21106000),
    FlashSector::create(0x21107000),
    FlashSector::create(0x21108000),
    FlashSector::create(0x21109000),
    FlashSector::create(0x2110a000),
    FlashSector::create(0x2110b000),
    FlashSector::create(0x2110c000),
    FlashSector::create(0x2110d000),
    FlashSector::create(0x2110e000),
    FlashSector::create(0x2110f000),
    FlashSector::create(0x21110000),
    FlashSector::create(0x21111000),
    FlashSector::create(0x21112000),
    FlashSector::create(0x21113000),
    FlashSector::create(0x21114000),
    FlashSector::create(0x21115000),
    FlashSector::create(0x21116000),
    FlashSector::create(0x21117000),
    FlashSector::create(0x21118000),
    FlashSector::create(0x21119000),
    FlashSector::create(0x2111a000),
    FlashSector::create(0x2111b000),
    FlashSector::create(0x2111c000),
    FlashSector::create(0x2111d000),
    FlashSector::create(0x2111e000),
    FlashSector::create(0x2111f000),
    FlashSector::create(0x21120000),
    FlashSector::create(0x21121000),
    FlashSector::create(0x21122000),
    FlashSector::create(0x21123000),
    FlashSector::create(0x21124000),
    FlashSector::create(0x21125000),
    FlashSector::create(0x21126000),
    FlashSector::create(0x21127000),
    FlashSector::create(0x21128000),
    FlashSector::create(0x21129000),
    FlashSector::create(0x2112a000),
    FlashSector::create(0x2112b000),
    FlashSector::create(0x2112c000),
    FlashSector::create(0x2112d000),
    FlashSector::create(0x2112e000),
    FlashSector::create(0x2112f000),
    FlashSector::create(0x21130000),
    FlashSector::create(0x21131000),
    FlashSector::create(0x21132000),
    FlashSector::create(0x21133000),
    FlashSector::create(0x21134000),
    FlashSector::create(0x21135000),
    FlashSector::create(0x21136000),
    FlashSector::create(0x21137000),
    FlashSector::create(0x21138000),
    FlashSector::create(0x21139000),
    FlashSector::create(0x2113a000),
    FlashSector::create(0x2113b000),
    FlashSector::create(0x2113c000),
    FlashSector::create(0x2113d000),
    FlashSector::create(0x2113e000),
    FlashSector::create(0x2113f000),
    FlashSector::create(0x21140000),
    FlashSector::create(0x21141000),
    FlashSector::create(0x21142000),
    FlashSector::create(0x21143000),
    FlashSector::create(0x21144000),
    FlashSector::create(0x21145000),
    FlashSector::create(0x21146000),
    FlashSector::create(0x21147000),
    FlashSector::create(0x21148000),
    FlashSector::create(0x21149000),
    FlashSector::create(0x2114a000),
    FlashSector::create(0x2114b000),
    FlashSector::create(0x2114c000),
    FlashSector::create(0x2114d000),
    FlashSector::create(0x2114e000),
    FlashSector::create(0x2114f000),
    FlashSector::create(0x21150000),
    FlashSector::create(0x21151000),
    FlashSector::create(0x21152000),
    FlashSector::create(0x21153000),
    FlashSector::create(0x21154000),
    FlashSector::create(0x21155000),
    FlashSector::create(0x21156000),
    FlashSector::create(0x21157000),
    FlashSector::create(0x21158000),
    FlashSector::create(0x21159000),
    FlashSector::create(0x2115a000),
    FlashSector::create(0x2115b000),
    FlashSector::create(0x2115c000),
    FlashSector::create(0x2115d000),
    FlashSector::create(0x2115e000),
    FlashSector::create(0x2115f000),
    FlashSector::create(0x21160000),
    FlashSector::create(0x21161000),
    FlashSector::create(0x21162000),
    FlashSector::create(0x21163000),
    FlashSector::create(0x21164000),
    FlashSector::create(0x21165000),
    FlashSector::create(0x21166000),
    FlashSector::create(0x21167000),
    FlashSector::create(0x21168000),
    FlashSector::create(0x21169000),
    FlashSector::create(0x2116a000),
    FlashSector::create(0x2116b000),
    FlashSector::create(0x2116c000),
    FlashSector::create(0x2116d000),
    FlashSector::create(0x2116e000),
    FlashSector::create(0x2116f000),
    FlashSector::create(0x21170000),
    FlashSector::create(0x21171000),
    FlashSector::create(0x21172000),
    FlashSector::create(0x21173000),
    FlashSector::create(0x21174000),
    FlashSector::create(0x21175000),
    FlashSector::create(0x21176000),
    FlashSector::create(0x21177000),
    FlashSector::create(0x21178000),
    FlashSector::create(0x21179000),
    FlashSector::create(0x2117a000),
    FlashSector::create(0x2117b000),
    FlashSector::create(0x2117c000),
    FlashSector::create(0x2117d000),
    FlashSector::create(0x2117e000),
    FlashSector::create(0x2117f000),
    FlashSector::create(0x21180000),
    FlashSector::create(0x21181000),
    FlashSector::create(0x21182000),
    FlashSector::create(0x21183000),
    FlashSector::create(0x21184000),
    FlashSector::create(0x21185000),
    FlashSector::create(0x21186000),
    FlashSector::create(0x21187000),
    FlashSector::create(0x21188000),
    FlashSector::create(0x21189000),
    FlashSector::create(0x2118a000),
    FlashSector::create(0x2118b000),
    FlashSector::create(0x2118c000),
    FlashSector::create(0x2118d000),
    FlashSector::create(0x2118e000),
    FlashSector::create(0x2118f000),
    FlashSector::create(0x21190000),
    FlashSector::create(0x21191000),
    FlashSector::create(0x21192000),
    FlashSector::create(0x21193000),
    FlashSector::create(0x21194000),
    FlashSector::create(0x21195000),
    FlashSector::create(0x21196000),
    FlashSector::create(0x21197000),
    FlashSector::create(0x21198000),
    FlashSector::create(0x21199000),
    FlashSector::create(0x2119a000),
    FlashSector::create(0x2119b000),
    FlashSector::create(0x2119c000),
    FlashSector::create(0x2119d000),
    FlashSector::create(0x2119e000),
    FlashSector::create(0x2119f000),
    FlashSector::create(0x211a0000),
    FlashSector::create(0x211a1000),
    FlashSector::create(0x211a2000),
    FlashSector::create(0x211a3000),
    FlashSector::create(0x211a4000),
    FlashSector::create(0x211a5000),
    FlashSector::create(0x211a6000),
    FlashSector::create(0x211a7000),
    FlashSector::create(0x211a8000),
    FlashSector::create(0x211a9000),
    FlashSector::create(0x211aa000),
    FlashSector::create(0x211ab000),
    FlashSector::create(0x211ac000),
    FlashSector::create(0x211ad000),
    FlashSector::create(0x211ae000),
    FlashSector::create(0x211af000),
    FlashSector::create(0x211b0000),
    FlashSector::create(0x211b1000),
    FlashSector::create(0x211b2000),
    FlashSector::create(0x211b3000),
    FlashSector::create(0x211b4000),
    FlashSector::create(0x211b5000),
    FlashSector::create(0x211b6000),
    FlashSector::create(0x211b7000),
    FlashSector::create(0x211b8000),
    FlashSector::create(0x211b9000),
    FlashSector::create(0x211ba000),
    FlashSector::create(0x211bb000),
    FlashSector::create(0x211bc000),
    FlashSector::create(0x211bd000),
    FlashSector::create(0x211be000),
    FlashSector::create(0x211bf000),
    FlashSector::create(0x211c0000),
    FlashSector::create(0x211c1000),
    FlashSector::create(0x211c2000),
    FlashSector::create(0x211c3000),
    FlashSector::create(0x211c4000),
    FlashSector::create(0x211c5000),
    FlashSector::create(0x211c6000),
    FlashSector::create(0x211c7000),
    FlashSector::create(0x211c8000),
    FlashSector::create(0x211c9000),
    FlashSector::create(0x211ca000),
    FlashSector::create(0x211cb000),
    FlashSector::create(0x211cc000),
    FlashSector::create(0x211cd000),
    FlashSector::create(0x211ce000),
    FlashSector::create(0x211cf000),
    FlashSector::create(0x211d0000),
    FlashSector::create(0x211d1000),
    FlashSector::create(0x211d2000),
    FlashSector::create(0x211d3000),
    FlashSector::create(0x211d4000),
    FlashSector::create(0x211d5000),
    FlashSector::create(0x211d6000),
    FlashSector::create(0x211d7000),
    FlashSector::create(0x211d8000),
    FlashSector::create(0x211d9000),
    FlashSector::create(0x211da000),
    FlashSector::create(0x211db000),
    FlashSector::create(0x211dc000),
    FlashSector::create(0x211dd000),
    FlashSector::create(0x211de000),
    FlashSector::create(0x211df000),
    FlashSector::create(0x211e0000),
    FlashSector::create(0x211e1000),
    FlashSector::create(0x211e2000),
    FlashSector::create(0x211e3000),
    FlashSector::create(0x211e4000),
    FlashSector::create(0x211e5000),
    FlashSector::create(0x211e6000),
    FlashSector::create(0x211e7000),
    FlashSector::create(0x211e8000),
    FlashSector::create(0x211e9000),
    FlashSector::create(0x211ea000),
    FlashSector::create(0x211eb000),
    FlashSector::create(0x211ec000),
    FlashSector::create(0x211ed000),
    FlashSector::create(0x211ee000),
    FlashSector::create(0x211ef000),
    FlashSector::create(0x211f0000),
    FlashSector::create(0x211f1000),
    FlashSector::create(0x211f2000),
    FlashSector::create(0x211f3000),
    FlashSector::create(0x211f4000),
    FlashSector::create(0x211f5000),
    FlashSector::create(0x211f6000),
    FlashSector::create(0x211f7000),
    FlashSector::create(0x211f8000),
    FlashSector::create(0x211f9000),
    FlashSector::create(0x211fa000),
    FlashSector::create(0x211fb000),
    FlashSector::create(0x211fc000),
    FlashSector::create(0x211fd000),
    FlashSector::create(0x211fe000),
    FlashSector::create(0x211ff000),
    FlashSector::create(0x21200000),
    FlashSector::create(0x21201000),
    FlashSector::create(0x21202000),
    FlashSector::create(0x21203000),
    FlashSector::create(0x21204000),
    FlashSector::create(0x21205000),
    FlashSector::create(0x21206000),
    FlashSector::create(0x21207000),
    FlashSector::create(0x21208000),
    FlashSector::create(0x21209000),
    FlashSector::create(0x2120a000),
    FlashSector::create(0x2120b000),
    FlashSector::create(0x2120c000),
    FlashSector::create(0x2120d000),
    FlashSector::create(0x2120e000),
    FlashSector::create(0x2120f000),
    FlashSector::create(0x21210000),
    FlashSector::create(0x21211000),
    FlashSector::create(0x21212000),
    FlashSector::create(0x21213000),
    FlashSector::create(0x21214000),
    FlashSector::create(0x21215000),
    FlashSector::create(0x21216000),
    FlashSector::create(0x21217000),
    FlashSector::create(0x21218000),
    FlashSector::create(0x21219000),
    FlashSector::create(0x2121a000),
    FlashSector::create(0x2121b000),
    FlashSector::create(0x2121c000),
    FlashSector::create(0x2121d000),
    FlashSector::create(0x2121e000),
    FlashSector::create(0x2121f000),
    FlashSector::create(0x21220000),
    FlashSector::create(0x21221000),
    FlashSector::create(0x21222000),
    FlashSector::create(0x21223000),
    FlashSector::create(0x21224000),
    FlashSector::create(0x21225000),
    FlashSector::create(0x21226000),
    FlashSector::create(0x21227000),
    FlashSector::create(0x21228000),
    FlashSector::create(0x21229000),
    FlashSector::create(0x2122a000),
    FlashSector::create(0x2122b000),
    FlashSector::create(0x2122c000),
    FlashSector::create(0x2122d000),
    FlashSector::create(0x2122e000),
    FlashSector::create(0x2122f000),
    FlashSector::create(0x21230000),
    FlashSector::create(0x21231000),
    FlashSector::create(0x21232000),
    FlashSector::create(0x21233000),
    FlashSector::create(0x21234000),
    FlashSector::create(0x21235000),
    FlashSector::create(0x21236000),
    FlashSector::create(0x21237000),
    FlashSector::create(0x21238000),
    FlashSector::create(0x21239000),
    FlashSector::create(0x2123a000),
    FlashSector::create(0x2123b000),
    FlashSector::create(0x2123c000),
    FlashSector::create(0x2123d000),
    FlashSector::create(0x2123e000),
    FlashSector::create(0x2123f000),
    FlashSector::create(0x21240000),
    FlashSector::create(0x21241000),
    FlashSector::create(0x21242000),
    FlashSector::create(0x21243000),
    FlashSector::create(0x21244000),
    FlashSector::create(0x21245000),
    FlashSector::create(0x21246000),
    FlashSector::create(0x21247000),
    FlashSector::create(0x21248000),
    FlashSector::create(0x21249000),
    FlashSector::create(0x2124a000),
    FlashSector::create(0x2124b000),
    FlashSector::create(0x2124c000),
    FlashSector::create(0x2124d000),
    FlashSector::create(0x2124e000),
    FlashSector::create(0x2124f000),
    FlashSector::create(0x21250000),
    FlashSector::create(0x21251000),
    FlashSector::create(0x21252000),
    FlashSector::create(0x21253000),
    FlashSector::create(0x21254000),
    FlashSector::create(0x21255000),
    FlashSector::create(0x21256000),
    FlashSector::create(0x21257000),
    FlashSector::create(0x21258000),
    FlashSector::create(0x21259000),
    FlashSector::create(0x2125a000),
    FlashSector::create(0x2125b000),
    FlashSector::create(0x2125c000),
    FlashSector::create(0x2125d000),
    FlashSector::create(0x2125e000),
    FlashSector::create(0x2125f000),
    FlashSector::create(0x21260000),
    FlashSector::create(0x21261000),
    FlashSector::create(0x21262000),
    FlashSector::create(0x21263000),
    FlashSector::create(0x21264000),
    FlashSector::create(0x21265000),
    FlashSector::create(0x21266000),
    FlashSector::create(0x21267000),
    FlashSector::create(0x21268000),
    FlashSector::create(0x21269000),
    FlashSector::create(0x2126a000),
    FlashSector::create(0x2126b000),
    FlashSector::create(0x2126c000),
    FlashSector::create(0x2126d000),
    FlashSector::create(0x2126e000),
    FlashSector::create(0x2126f000),
    FlashSector::create(0x21270000),
    FlashSector::create(0x21271000),
    FlashSector::create(0x21272000),
    FlashSector::create(0x21273000),
    FlashSector::create(0x21274000),
    FlashSector::create(0x21275000),
    FlashSector::create(0x21276000),
    FlashSector::create(0x21277000),
    FlashSector::create(0x21278000),
    FlashSector::create(0x21279000),
    FlashSector::create(0x2127a000),
    FlashSector::create(0x2127b000),
    FlashSector::create(0x2127c000),
    FlashSector::create(0x2127d000),
    FlashSector::create(0x2127e000),
    FlashSector::create(0x2127f000),
    FlashSector::create(0x21280000),
    FlashSector::create(0x21281000),
    FlashSector::create(0x21282000),
    FlashSector::create(0x21283000),
    FlashSector::create(0x21284000),
    FlashSector::create(0x21285000),
    FlashSector::create(0x21286000),
    FlashSector::create(0x21287000),
    FlashSector::create(0x21288000),
    FlashSector::create(0x21289000),
    FlashSector::create(0x2128a000),
    FlashSector::create(0x2128b000),
    FlashSector::create(0x2128c000),
    FlashSector::create(0x2128d000),
    FlashSector::create(0x2128e000),
    FlashSector::create(0x2128f000),
    FlashSector::create(0x21290000),
    FlashSector::create(0x21291000),
    FlashSector::create(0x21292000),
    FlashSector::create(0x21293000),
    FlashSector::create(0x21294000),
    FlashSector::create(0x21295000),
    FlashSector::create(0x21296000),
    FlashSector::create(0x21297000),
    FlashSector::create(0x21298000),
    FlashSector::create(0x21299000),
    FlashSector::create(0x2129a000),
    FlashSector::create(0x2129b000),
    FlashSector::create(0x2129c000),
    FlashSector::create(0x2129d000),
    FlashSector::create(0x2129e000),
    FlashSector::create(0x2129f000),
    FlashSector::create(0x212a0000),
    FlashSector::create(0x212a1000),
    FlashSector::create(0x212a2000),
    FlashSector::create(0x212a3000),
    FlashSector::create(0x212a4000),
    FlashSector::create(0x212a5000),
    FlashSector::create(0x212a6000),
    FlashSector::create(0x212a7000),
    FlashSector::create(0x212a8000),
    FlashSector::create(0x212a9000),
    FlashSector::create(0x212aa000),
    FlashSector::create(0x212ab000),
    FlashSector::create(0x212ac000),
    FlashSector::create(0x212ad000),
    FlashSector::create(0x212ae000),
    FlashSector::create(0x212af000),
    FlashSector::create(0x212b0000),
    FlashSector::create(0x212b1000),
    FlashSector::create(0x212b2000),
    FlashSector::create(0x212b3000),
    FlashSector::create(0x212b4000),
    FlashSector::create(0x212b5000),
    FlashSector::create(0x212b6000),
    FlashSector::create(0x212b7000),
    FlashSector::create(0x212b8000),
    FlashSector::create(0x212b9000),
    FlashSector::create(0x212ba000),
    FlashSector::create(0x212bb000),
    FlashSector::create(0x212bc000),
    FlashSector::create(0x212bd000),
    FlashSector::create(0x212be000),
    FlashSector::create(0x212bf000),
    FlashSector::create(0x212c0000),
    FlashSector::create(0x212c1000),
    FlashSector::create(0x212c2000),
    FlashSector::create(0x212c3000),
    FlashSector::create(0x212c4000),
    FlashSector::create(0x212c5000),
    FlashSector::create(0x212c6000),
    FlashSector::create(0x212c7000),
    FlashSector::create(0x212c8000),
    FlashSector::create(0x212c9000),
    FlashSector::create(0x212ca000),
    FlashSector::create(0x212cb000),
    FlashSector::create(0x212cc000),
    FlashSector::create(0x212cd000),
    FlashSector::create(0x212ce000),
    FlashSector::create(0x212cf000),
    FlashSector::create(0x212d0000),
    FlashSector::create(0x212d1000),
    FlashSector::create(0x212d2000),
    FlashSector::create(0x212d3000),
    FlashSector::create(0x212d4000),
    FlashSector::create(0x212d5000),
    FlashSector::create(0x212d6000),
    FlashSector::create(0x212d7000),
    FlashSector::create(0x212d8000),
    FlashSector::create(0x212d9000),
    FlashSector::create(0x212da000),
    FlashSector::create(0x212db000),
    FlashSector::create(0x212dc000),
    FlashSector::create(0x212dd000),
    FlashSector::create(0x212de000),
    FlashSector::create(0x212df000),
    FlashSector::create(0x212e0000),
    FlashSector::create(0x212e1000),
    FlashSector::create(0x212e2000),
    FlashSector::create(0x212e3000),
    FlashSector::create(0x212e4000),
    FlashSector::create(0x212e5000),
    FlashSector::create(0x212e6000),
    FlashSector::create(0x212e7000),
    FlashSector::create(0x212e8000),
    FlashSector::create(0x212e9000),
    FlashSector::create(0x212ea000),
    FlashSector::create(0x212eb000),
    FlashSector::create(0x212ec000),
    FlashSector::create(0x212ed000),
    FlashSector::create(0x212ee000),
    FlashSector::create(0x212ef000),
    FlashSector::create(0x212f0000),
    FlashSector::create(0x212f1000),
    FlashSector::create(0x212f2000),
    FlashSector::create(0x212f3000),
    FlashSector::create(0x212f4000),
    FlashSector::create(0x212f5000),
    FlashSector::create(0x212f6000),
    FlashSector::create(0x212f7000),
    FlashSector::create(0x212f8000),
    FlashSector::create(0x212f9000),
    FlashSector::create(0x212fa000),
    FlashSector::create(0x212fb000),
    FlashSector::create(0x212fc000),
    FlashSector::create(0x212fd000),
    FlashSector::create(0x212fe000),
    FlashSector::create(0x212ff000),
    FlashSector::create(0x21300000),
    FlashSector::create(0x21301000),
    FlashSector::create(0x21302000),
    FlashSector::create(0x21303000),
    FlashSector::create(0x21304000),
    FlashSector::create(0x21305000),
    FlashSector::create(0x21306000),
    FlashSector::create(0x21307000),
    FlashSector::create(0x21308000),
    FlashSector::create(0x21309000),
    FlashSector::create(0x2130a000),
    FlashSector::create(0x2130b000),
    FlashSector::create(0x2130c000),
    FlashSector::create(0x2130d000),
    FlashSector::create(0x2130e000),
    FlashSector::create(0x2130f000),
    FlashSector::create(0x21310000),
    FlashSector::create(0x21311000),
    FlashSector::create(0x21312000),
    FlashSector::create(0x21313000),
    FlashSector::create(0x21314000),
    FlashSector::create(0x21315000),
    FlashSector::create(0x21316000),
    FlashSector::create(0x21317000),
    FlashSector::create(0x21318000),
    FlashSector::create(0x21319000),
    FlashSector::create(0x2131a000),
    FlashSector::create(0x2131b000),
    FlashSector::create(0x2131c000),
    FlashSector::create(0x2131d000),
    FlashSector::create(0x2131e000),
    FlashSector::create(0x2131f000),
    FlashSector::create(0x21320000),
    FlashSector::create(0x21321000),
    FlashSector::create(0x21322000),
    FlashSector::create(0x21323000),
    FlashSector::create(0x21324000),
    FlashSector::create(0x21325000),
    FlashSector::create(0x21326000),
    FlashSector::create(0x21327000),
    FlashSector::create(0x21328000),
    FlashSector::create(0x21329000),
    FlashSector::create(0x2132a000),
    FlashSector::create(0x2132b000),
    FlashSector::create(0x2132c000),
    FlashSector::create(0x2132d000),
    FlashSector::create(0x2132e000),
    FlashSector::create(0x2132f000),
    FlashSector::create(0x21330000),
    FlashSector::create(0x21331000),
    FlashSector::create(0x21332000),
    FlashSector::create(0x21333000),
    FlashSector::create(0x21334000),
    FlashSector::create(0x21335000),
    FlashSector::create(0x21336000),
    FlashSector::create(0x21337000),
    FlashSector::create(0x21338000),
    FlashSector::create(0x21339000),
    FlashSector::create(0x2133a000),
    FlashSector::create(0x2133b000),
    FlashSector::create(0x2133c000),
    FlashSector::create(0x2133d000),
    FlashSector::create(0x2133e000),
    FlashSector::create(0x2133f000),
    FlashSector::create(0x21340000),
    FlashSector::create(0x21341000),
    FlashSector::create(0x21342000),
    FlashSector::create(0x21343000),
    FlashSector::create(0x21344000),
    FlashSector::create(0x21345000),
    FlashSector::create(0x21346000),
    FlashSector::create(0x21347000),
    FlashSector::create(0x21348000),
    FlashSector::create(0x21349000),
    FlashSector::create(0x2134a000),
    FlashSector::create(0x2134b000),
    FlashSector::create(0x2134c000),
    FlashSector::create(0x2134d000),
    FlashSector::create(0x2134e000),
    FlashSector::create(0x2134f000),
    FlashSector::create(0x21350000),
    FlashSector::create(0x21351000),
    FlashSector::create(0x21352000),
    FlashSector::create(0x21353000),
    FlashSector::create(0x21354000),
    FlashSector::create(0x21355000),
    FlashSector::create(0x21356000),
    FlashSector::create(0x21357000),
    FlashSector::create(0x21358000),
    FlashSector::create(0x21359000),
    FlashSector::create(0x2135a000),
    FlashSector::create(0x2135b000),
    FlashSector::create(0x2135c000),
    FlashSector::create(0x2135d000),
    FlashSector::create(0x2135e000),
    FlashSector::create(0x2135f000),
    FlashSector::create(0x21360000),
    FlashSector::create(0x21361000),
    FlashSector::create(0x21362000),
    FlashSector::create(0x21363000),
    FlashSector::create(0x21364000),
    FlashSector::create(0x21365000),
    FlashSector::create(0x21366000),
    FlashSector::create(0x21367000),
    FlashSector::create(0x21368000),
    FlashSector::create(0x21369000),
    FlashSector::create(0x2136a000),
    FlashSector::create(0x2136b000),
    FlashSector::create(0x2136c000),
    FlashSector::create(0x2136d000),
    FlashSector::create(0x2136e000),
    FlashSector::create(0x2136f000),
    FlashSector::create(0x21370000),
    FlashSector::create(0x21371000),
    FlashSector::create(0x21372000),
    FlashSector::create(0x21373000),
    FlashSector::create(0x21374000),
    FlashSector::create(0x21375000),
    FlashSector::create(0x21376000),
    FlashSector::create(0x21377000),
    FlashSector::create(0x21378000),
    FlashSector::create(0x21379000),
    FlashSector::create(0x2137a000),
    FlashSector::create(0x2137b000),
    FlashSector::create(0x2137c000),
    FlashSector::create(0x2137d000),
    FlashSector::create(0x2137e000),
    FlashSector::create(0x2137f000),
    FlashSector::create(0x21380000),
    FlashSector::create(0x21381000),
    FlashSector::create(0x21382000),
    FlashSector::create(0x21383000),
    FlashSector::create(0x21384000),
    FlashSector::create(0x21385000),
    FlashSector::create(0x21386000),
    FlashSector::create(0x21387000),
    FlashSector::create(0x21388000),
    FlashSector::create(0x21389000),
    FlashSector::create(0x2138a000),
    FlashSector::create(0x2138b000),
    FlashSector::create(0x2138c000),
    FlashSector::create(0x2138d000),
    FlashSector::create(0x2138e000),
    FlashSector::create(0x2138f000),
    FlashSector::create(0x21390000),
    FlashSector::create(0x21391000),
    FlashSector::create(0x21392000),
    FlashSector::create(0x21393000),
    FlashSector::create(0x21394000),
    FlashSector::create(0x21395000),
    FlashSector::create(0x21396000),
    FlashSector::create(0x21397000),
    FlashSector::create(0x21398000),
    FlashSector::create(0x21399000),
    FlashSector::create(0x2139a000),
    FlashSector::create(0x2139b000),
    FlashSector::create(0x2139c000),
    FlashSector::create(0x2139d000),
    FlashSector::create(0x2139e000),
    FlashSector::create(0x2139f000),
    FlashSector::create(0x213a0000),
    FlashSector::create(0x213a1000),
    FlashSector::create(0x213a2000),
    FlashSector::create(0x213a3000),
    FlashSector::create(0x213a4000),
    FlashSector::create(0x213a5000),
    FlashSector::create(0x213a6000),
    FlashSector::create(0x213a7000),
    FlashSector::create(0x213a8000),
    FlashSector::create(0x213a9000),
    FlashSector::create(0x213aa000),
    FlashSector::create(0x213ab000),
    FlashSector::create(0x213ac000),
    FlashSector::create(0x213ad000),
    FlashSector::create(0x213ae000),
    FlashSector::create(0x213af000),
    FlashSector::create(0x213b0000),
    FlashSector::create(0x213b1000),
    FlashSector::create(0x213b2000),
    FlashSector::create(0x213b3000),
    FlashSector::create(0x213b4000),
    FlashSector::create(0x213b5000),
    FlashSector::create(0x213b6000),
    FlashSector::create(0x213b7000),
    FlashSector::create(0x213b8000),
    FlashSector::create(0x213b9000),
    FlashSector::create(0x213ba000),
    FlashSector::create(0x213bb000),
    FlashSector::create(0x213bc000),
    FlashSector::create(0x213bd000),
    FlashSector::create(0x213be000),
    FlashSector::create(0x213bf000),
    FlashSector::create(0x213c0000),
    FlashSector::create(0x213c1000),
    FlashSector::create(0x213c2000),
    FlashSector::create(0x213c3000),
    FlashSector::create(0x213c4000),
    FlashSector::create(0x213c5000),
    FlashSector::create(0x213c6000),
    FlashSector::create(0x213c7000),
    FlashSector::create(0x213c8000),
    FlashSector::create(0x213c9000),
    FlashSector::create(0x213ca000),
    FlashSector::create(0x213cb000),
    FlashSector::create(0x213cc000),
    FlashSector::create(0x213cd000),
    FlashSector::create(0x213ce000),
    FlashSector::create(0x213cf000),
    FlashSector::create(0x213d0000),
    FlashSector::create(0x213d1000),
    FlashSector::create(0x213d2000),
    FlashSector::create(0x213d3000),
    FlashSector::create(0x213d4000),
    FlashSector::create(0x213d5000),
    FlashSector::create(0x213d6000),
    FlashSector::create(0x213d7000),
    FlashSector::create(0x213d8000),
    FlashSector::create(0x213d9000),
    FlashSector::create(0x213da000),
    FlashSector::create(0x213db000),
    FlashSector::create(0x213dc000),
    FlashSector::create(0x213dd000),
    FlashSector::create(0x213de000),
    FlashSector::create(0x213df000),
    FlashSector::create(0x213e0000),
    FlashSector::create(0x213e1000),
    FlashSector::create(0x213e2000),
    FlashSector::create(0x213e3000),
    FlashSector::create(0x213e4000),
    FlashSector::create(0x213e5000),
    FlashSector::create(0x213e6000),
    FlashSector::create(0x213e7000),
    FlashSector::create(0x213e8000),
    FlashSector::create(0x213e9000),
    FlashSector::create(0x213ea000),
    FlashSector::create(0x213eb000),
    FlashSector::create(0x213ec000),
    FlashSector::create(0x213ed000),
    FlashSector::create(0x213ee000),
    FlashSector::create(0x213ef000),
    FlashSector::create(0x213f0000),
    FlashSector::create(0x213f1000),
    FlashSector::create(0x213f2000),
    FlashSector::create(0x213f3000),
    FlashSector::create(0x213f4000),
    FlashSector::create(0x213f5000),
    FlashSector::create(0x213f6000),
    FlashSector::create(0x213f7000),
    FlashSector::create(0x213f8000),
    FlashSector::create(0x213f9000),
    FlashSector::create(0x213fa000),
    FlashSector::create(0x213fb000),
    FlashSector::create(0x213fc000),
    FlashSector::create(0x213fd000),
    FlashSector::create(0x213fe000),
    FlashSector::create(0x213ff000),
];