use lazy_static::lazy_static;

use pastel::Color;
use crate::parser::parse_color;


/// color name in chinese
#[derive(Debug, Clone)]
pub struct NamedColorZh {
    pub name: &'static str,
    pub name_zh: &'static str,
    pub color: Color,
    pub hex_color: &'static str,
}

pub fn name_color_zh(name: &'static str, name_zh:&'static str, color:&'static str) -> NamedColorZh {
    NamedColorZh {
        name,
        name_zh,
        color:parse_color(color).unwrap(),
        hex_color:color,
    }
}

lazy_static!{
    pub static ref NAMED_COLORS_ZH: [NamedColorZh; 526] = [
        name_color_zh("anyuzi","暗玉紫","#5c2223"),
        name_color_zh("mudanfenhong","牡丹粉红","#eea2a4"),
        name_color_zh("lizi","栗紫","#5a191b"),
        name_color_zh("xiangyehong","香叶红","#f07c82"),
        name_color_zh("putaojiangzi","葡萄酱紫","#5a1216"),
        name_color_zh("yanhong","艳红","#ed5a65"),
        name_color_zh("yuhong","玉红","#c04851"),
        name_color_zh("chahuahong","茶花红","#ee3f4d"),
        name_color_zh("gaolianghong","高粱红","#c02c38"),
        name_color_zh("manjianghong","满江红","#a7535a"),
        name_color_zh("shubihong","鼠鼻红","#e3b4b8"),
        name_color_zh("hehuanhong","合欢红","#f0a1a8"),
        name_color_zh("chunmeihong","春梅红","#f1939c"),
        name_color_zh("xiancaihong","苋菜红","#a61b29"),
        name_color_zh("yanhong","烟红","#894e54"),
        name_color_zh("meihong","莓红","#c45a65"),
        name_color_zh("eguanhong","鹅冠红","#d11a2d"),
        name_color_zh("fengyehong","枫叶红","#c21f30"),
        name_color_zh("tangchangpuhong","唐菖蒲红","#de1c31"),
        name_color_zh("zaohong","枣红","#7c1823"),
        name_color_zh("zhuganzi","猪肝紫","#541e24"),
        name_color_zh("putaozi","葡萄紫","#4c1f24"),
        name_color_zh("anziyuanhong","暗紫苑红","#82202b"),
        name_color_zh("yanhong","殷红","#82111f"),
        name_color_zh("caomolihong","草茉莉红","#ef475d"),
        name_color_zh("jiangzi","酱紫","#4d1018"),
        name_color_zh("shanchahong","山茶红","#ed556a"),
        name_color_zh("xinhui","锌灰","#7a7374"),
        name_color_zh("haitanghong","海棠红","#f03752"),
        name_color_zh("jifenhong","蓟粉红","#e6d2d5"),
        name_color_zh("shiruihong","石蕊红","#f0c9cf"),
        name_color_zh("danshuhong","淡曙红","#ee2746"),
        name_color_zh("lizi","李紫","#2b1216"),
        name_color_zh("shizhuhong","石竹红","#ee4863"),
        name_color_zh("danqianhong","淡茜红","#e77c8e"),
        name_color_zh("jinyuzi","金鱼紫","#500a16"),
        name_color_zh("shanlidouhong","山黎豆红","#c27c88"),
        name_color_zh("shubeihui","鼠背灰","#73575c"),
        name_color_zh("danruixianghong","淡蕊香红","#ee4866"),
        name_color_zh("ganzhezi","甘蔗紫","#621624"),
        name_color_zh("yuejihong","月季红","#ce5777"),
        name_color_zh("jianjingyuhong","尖晶玉红","#cc163a"),
        name_color_zh("shuihong","水红","#f1c4cd"),
        name_color_zh("jianghong","姜红","#eeb8c3"),
        name_color_zh("luhui","芦灰","#856d72"),
        name_color_zh("jiapizi","茄皮紫","#2d0c13"),
        name_color_zh("diaozhonghuahong","吊钟花红","#2d0c13"),
        name_color_zh("cangyinghui","苍蝇灰","#36282b"),
        name_color_zh("jinkuihong","锦葵红","#bf3553"),
        name_color_zh("fentuanhuahong","粉团花红","#ec9bad"),
        name_color_zh("shizhuzi","石竹紫","#63071c"),
        name_color_zh("luanshizi","卵石紫","#30161c"),
        name_color_zh("jinghong","晶红","#eea6b7"),
        name_color_zh("shaoyaogenghong","芍药耕红","#eba0b3"),
        name_color_zh("muyunhui","暮云灰","#4f383e"),
        name_color_zh("jiangdouhong","豇豆红","#ed9db2"),
        name_color_zh("baochunhong","报春红","#ec8aa4"),
        name_color_zh("danjianghong","淡绛红","#ec7696"),
        name_color_zh("fengxianhuahong","凤仙花红","#ea7293"),
        name_color_zh("xiaguanghong","霞光红","#ef82a0"),
        name_color_zh("xidanhong","喜蛋红","#ec2c64"),
        name_color_zh("jiazhutaohong","夹竹桃红","#eb507e"),
        name_color_zh("songyemudanhong","松叶牡丹红","#eb3c70"),
        name_color_zh("lianbanhong","莲瓣红","#ea517f"),
        name_color_zh("baijihong","白芨红","#de7897"),
        name_color_zh("yinhonghui","隐红灰","#b598a1"),
        name_color_zh("wenpochuan","榲桲舡","#ed2f6a"),
        name_color_zh("cujiangcaohong","酢酱草红","#c5708b"),
        name_color_zh("huoezi","火鹅紫","#33141e"),
        name_color_zh("yaoguanzi","鹞冠紫","#621d34"),
        name_color_zh("pinhong","品红","#ef3473"),
        name_color_zh("moshizi","磨石紫","#382129"),
        name_color_zh("mozi","墨紫","#310f1b"),
        name_color_zh("tanzi","檀紫","#381924"),
        name_color_zh("chuhehong","初荷红","#e16c96"),
        name_color_zh("caitouzi","菜头紫","#951c48"),
        name_color_zh("putaojiuhong","葡萄酒红","#62102e"),
        name_color_zh("danqingzi","淡青紫","#e0c8d1"),
        name_color_zh("bogenhong","菠根红","#d13c74"),
        name_color_zh("haixiangzi","海象紫","#4b1e2f"),
        name_color_zh("tuyanhong","兔眼红","#ec4e8a"),
        name_color_zh("nenlinghong","嫩菱红","#de3f7c"),
        name_color_zh("yangcongzi","洋葱紫","#a8456b"),
        name_color_zh("ganzi","绀紫","#461629"),
        name_color_zh("zijinghong","紫荆红","#ee2c79"),
        name_color_zh("biandouhuahong","扁豆花红","#ef498b"),
        name_color_zh("mabiancaozi","马鞭草紫","#ede3e7"),
        name_color_zh("canghuahong","藏花红","#ec2d7a"),
        name_color_zh("banjiuhui","斑鸠灰","#482936"),
        name_color_zh("gutongzi","古铜紫","#440e25"),
        name_color_zh("danzihong","丹紫红","#d2568c"),
        name_color_zh("dingxiangdanzi","丁香淡紫","#e9d7df"),
        name_color_zh("meiguihong","玫瑰红","#d2357d"),
        name_color_zh("gudinghui","古鼎灰","#36292f"),
        name_color_zh("lingmenghong","菱锰红","#d276a3"),
        name_color_zh("yingcaozi","樱草紫","#c06f98"),
        name_color_zh("longxuhong","龙须红","#cc5595"),
        name_color_zh("dianqishihong","电气石红","#c35691"),
        name_color_zh("meiguizi","玫瑰紫","#ba2f7b"),
        name_color_zh("xiancaizi","苋菜紫","#9b1e64"),
        name_color_zh("zihui","紫灰","#5d3f51"),
        name_color_zh("longjingyuzi","龙睛鱼紫","#4e2a40"),
        name_color_zh("qinghakezi","青蛤壳紫","#bc84a8"),
        name_color_zh("luolanzi","萝兰紫","#c08eaf"),
        name_color_zh("biqizi","荸荠紫","#411c35"),
        name_color_zh("doukouzi","豆蔻紫","#ad6598"),
        name_color_zh("biandouzi","扁豆紫","#a35c8f"),
        name_color_zh("qianniuzi","牵牛紫","#681752"),
        name_color_zh("zizi","芓紫","#894276"),
        name_color_zh("gejinzi","葛巾紫","#7e2065"),
        name_color_zh("qinglian","青莲","#8b2671"),
        name_color_zh("jiehuazi","芥花紫","#983680"),
        name_color_zh("fengxinzi","凤信紫","#c8adc4"),
        name_color_zh("shenqianniuzi","深牵牛紫","#1c0d1a"),
        name_color_zh("zhilanzi","芝兰紫","#7e1671"),
        name_color_zh("weizi","魏紫","#7e1671"),
        name_color_zh("niaomeizi","鸟梅紫","#1e131d"),
        name_color_zh("jiegengzi","桔梗紫","#813c85"),
        name_color_zh("danqianniuzi","淡牵牛紫","#d1c2d3"),
        name_color_zh("jianfengzi","剑锋紫","#3e3841"),
        name_color_zh("xunzi","蕈紫","#815c94"),
        name_color_zh("jinzi","槿紫","#806d9e"),
        name_color_zh("qianshibai","芡食白","#e2e1e4"),
        name_color_zh("longkuizi","龙葵紫","#322f3b"),
        name_color_zh("tengluozi","藤萝紫","#8076a3"),
        name_color_zh("shayuhui","沙鱼灰","#35333c"),
        name_color_zh("anlongdanzi","暗龙胆紫","#22202e"),
        name_color_zh("anlanzi","暗蓝紫","#131124"),
        name_color_zh("yeputaozi","野葡萄紫","#302f4b"),
        name_color_zh("yejuzi","野菊紫","#525288"),
        name_color_zh("shuiniuhui","水牛灰","#2f2f35"),
        name_color_zh("yuanshanzi","远山紫","#ccccd6"),
        name_color_zh("luodianzi","螺甸紫","#74759b"),
        name_color_zh("jingshizi","晶石紫","#1f2040"),
        name_color_zh("mantianxingzi","满天星紫","#2e317c"),
        name_color_zh("danlanzi","淡蓝紫","#a7a8bd"),
        name_color_zh("shangengzi","山梗紫","#61649f"),
        name_color_zh("niujiaohui","牛角灰","#2d2e36"),
        name_color_zh("yuweihui","鱼尾灰","#5e616d"),
        name_color_zh("waguanhui","瓦罐灰","#47484c"),
        name_color_zh("ganglan","钢蓝","#0f1423"),
        name_color_zh("yanhanlan","燕颔蓝","#131824"),
        name_color_zh("jingyuhui","鲸鱼灰","#475164"),
        name_color_zh("qinghui","青灰","#2b333e"),
        name_color_zh("gelan","鸽蓝","#1c2938"),
        name_color_zh("anlan","暗蓝","#101f30"),
        name_color_zh("gangqing","钢青","#142334"),
        name_color_zh("haitaolan","海涛蓝","#15559a"),
        name_color_zh("feiyancaolan","飞燕草蓝","#0f59a4"),
        name_color_zh("dianqing","靛青","#1661ab"),
        name_color_zh("ananlan","安安蓝","#3170a7"),
        name_color_zh("haijunlan","海军蓝","#346c9c"),
        name_color_zh("jingtailan","景泰蓝","#2775b6"),
        name_color_zh("pinlan","品蓝","#2b73af"),
        name_color_zh("niluolan","尼罗蓝","#2474b5"),
        name_color_zh("diechilan","蝶翅蓝","#4e7ca1"),
        name_color_zh("huaqing","花青","#2376b7"),
        name_color_zh("yanlan","鷃蓝","#144a74"),
        name_color_zh("xinglan","星蓝","#93b5cf"),
        name_color_zh("honglan","虹蓝","#2177b8"),
        name_color_zh("bolinlan","柏林蓝","#126bae"),
        name_color_zh("qunqing","群青","#1772b4"),
        name_color_zh("yunshuilan","云水蓝","#baccd9"),
        name_color_zh("yushandoulan","羽扇豆蓝","#619ac3"),
        name_color_zh("zhanjianhui","战舰灰","#495c69"),
        name_color_zh("qingshanlan","晴山蓝","#8fb2c9"),
        name_color_zh("jinglan","睛蓝","#5698c3"),
        name_color_zh("tangcilan","搪磁蓝","#11659a"),
        name_color_zh("chaolan","潮蓝","#2983bb"),
        name_color_zh("tianlan","天蓝","#1677b3"),
        name_color_zh("dalishihui","大理石灰","#c4cbcf"),
        name_color_zh("qianniuhualan","牵牛花蓝","#1177b0"),
        name_color_zh("baoshilan","宝石蓝","#2486b9"),
        name_color_zh("danlanhui","淡蓝灰","#5e7987"),
        name_color_zh("nenhui","嫩灰","#74787a"),
        name_color_zh("yinyubai","银鱼白","#cdd1d3"),
        name_color_zh("youlan","釉蓝","#1781b5"),
        name_color_zh("jianshilan","涧石蓝","#66a9c9"),
        name_color_zh("yuantianlan","远天蓝","#d0dfe6"),
        name_color_zh("yunshanlan","云山蓝","#2f90b9"),
        name_color_zh("qiubolan","秋波蓝","#8abcd1"),
        name_color_zh("jingtianlan","井天蓝","#c3d7df"),
        name_color_zh("yuanweilan","鸢尾蓝","#158bb8"),
        name_color_zh("yunfengbai","云峰白","#d8e3e7"),
        name_color_zh("xinghui","星灰","#b2bbbe"),
        name_color_zh("gulan","钴蓝","#1a94bc"),
        name_color_zh("biqing","碧青","#5cb3cc"),
        name_color_zh("canglan","苍蓝","#134857"),
        name_color_zh("shenhuilan","深灰蓝","#132c33"),
        name_color_zh("huilan","灰蓝","#21373d"),
        name_color_zh("hushuilan","湖水蓝","#b0d5df"),
        name_color_zh("haiqing","海青","#22a2c3"),
        name_color_zh("huanghunhui","黄昏灰","#474b4c"),
        name_color_zh("jiqing","霁青","#63bbd0"),
        name_color_zh("yuqinlan","玉鈫蓝","#126e82"),
        name_color_zh("danfanlan","胆矾蓝","#0f95b0"),
        name_color_zh("jianniaolan","樫鸟蓝","#1491a8"),
        name_color_zh("oulan","鸥蓝","#c7d2d4"),
        name_color_zh("cuilan","翠蓝","#1e9eb3"),
        name_color_zh("qingtinglan","蜻蜓蓝","#3b818c"),
        name_color_zh("kongquelan","孔雀蓝","#0eb0c9"),
        name_color_zh("weilan","蔚蓝","#29b7cb"),
        name_color_zh("pubulan","瀑布蓝","#51c4d3"),
        name_color_zh("shanlan","闪蓝","#7cabb1"),
        name_color_zh("dianzilan","甸子蓝","#10aec2"),
        name_color_zh("wanbolan","晚波蓝","#648e93"),
        name_color_zh("qingshuilan","清水蓝","#93d5dc"),
        name_color_zh("xiayunhui","夏云灰","#617172"),
        name_color_zh("haitianlan","海天蓝","#c6e6e8"),
        name_color_zh("xiakeqing","虾壳青","#869d9d"),
        name_color_zh("shilv","石绿","#57c3c2"),
        name_color_zh("qionghui","穹灰","#c4d7d6"),
        name_color_zh("meidielv","美蝶绿","#12aa9c"),
        name_color_zh("ehui","垩灰","#737c7b"),
        name_color_zh("lanlv","蓝绿","#12a182"),
        name_color_zh("zhulv","竹绿","#1ba784"),
        name_color_zh("yadinglv","亚丁绿","#428675"),
        name_color_zh("yueyingbai","月影白","#c0c4c3"),
        name_color_zh("haiwanglv","海王绿","#248067"),
        name_color_zh("shenhailv","深海绿","#1a3b32"),
        name_color_zh("lvhui","绿灰","#314a43"),
        name_color_zh("qingfanlv","青矾绿","#2c9678"),
        name_color_zh("canglv","苍绿","#223e36"),
        name_color_zh("feiquanlv","飞泉绿","#497568"),
        name_color_zh("mangconglv","莽丛绿","#141e1b"),
        name_color_zh("wuzhilv","梧枝绿","#69a794"),
        name_color_zh("tonglv","铜绿","#2bae85"),
        name_color_zh("caoyuanyuanlv","草原远绿","#9abeaf"),
        name_color_zh("walv","蛙绿","#45b787"),
        name_color_zh("langhualv","浪花绿","#92b3a5"),
        name_color_zh("ganlanlv","苷蓝绿","#1f2623"),
        name_color_zh("fenlv","粉绿","#83cbac"),
        name_color_zh("danlvhui","淡绿灰","#70887d"),
        name_color_zh("maimiaolv","麦苗绿","#55bb8a"),
        name_color_zh("cuilv","翠绿","#20a162"),
        name_color_zh("conglv","葱绿","#40a070"),
        name_color_zh("heyelv","荷叶绿","#1a6840"),
        name_color_zh("danlv","淡绿","#61ac85"),
        name_color_zh("tianyuanlv","田园绿","#68b88e"),
        name_color_zh("yuzanlv","玉簪绿","#a4cab6"),
        name_color_zh("chanlv","蟾绿","#3c9566"),
        name_color_zh("koushaolv","蔻梢绿","#5dbe8a"),
        name_color_zh("bohelv","薄荷绿","#207f4c"),
        name_color_zh("yuebai","月白","#eef7f2"),
        name_color_zh("danbaishilv","蛋白石绿","#579572"),
        name_color_zh("zhuhuanglv","竹篁绿","#b9dec9"),
        name_color_zh("kongquelv","孔雀绿","#229453"),
        name_color_zh("gongdianlv","宫殿绿","#20894d"),
        name_color_zh("yunshanlv","云杉绿","#15231b"),
        name_color_zh("maolv","毛绿","#66c18c"),
        name_color_zh("bingshanlan","冰山蓝","#a4aca7"),
        name_color_zh("minghui","明灰","#8a988e"),
        name_color_zh("minglv","明绿","#9eccab"),
        name_color_zh("songshuanglv","松霜绿","#83a78d"),
        name_color_zh("baiqucailv","白屈菜绿","#485b4d"),
        name_color_zh("langyanhui","狼烟灰","#5d655f"),
        name_color_zh("wasonglv","瓦松绿","#6e8b74"),
        name_color_zh("hujishenglv","槲寄生绿","#2b312c"),
        name_color_zh("dancuilv","淡翠绿","#c6dfc8"),
        name_color_zh("yusuilv","玉髓绿","#41b349"),
        name_color_zh("xianlv","鲜绿","#43b244"),
        name_color_zh("youlv","油绿","#253d24"),
        name_color_zh("baoshilv","宝石绿","#41ae3c"),
        name_color_zh("jialingshuilv","嘉陵水绿","#add5a2"),
        name_color_zh("tianluolv","田螺绿","#5e665b"),
        name_color_zh("shuilv","水绿","#8cc269"),
        name_color_zh("yingwulv","鹦鹉绿","#5bae23"),
        name_color_zh("aibeilv","艾背绿","#dfecd5"),
        name_color_zh("ailv","艾绿","#cad3c3"),
        name_color_zh("niehui","镍灰","#9fa39a"),
        name_color_zh("ganlanshilv","橄榄石绿","#b2cf87"),
        name_color_zh("yalv","芽绿","#96c24e"),
        name_color_zh("nenjulv","嫩菊绿","#f0f5e5"),
        name_color_zh("luweilv","芦苇绿","#b7d07a"),
        name_color_zh("yaohuang","姚黄","#d0deaa"),
        name_color_zh("enyoulv","蒽油绿","#373834"),
        name_color_zh("pingguolv","苹果绿","#bacf65"),
        name_color_zh("haimeilv","海沬绿","#e2e7bf"),
        name_color_zh("ganlanhuanglv","橄榄黄绿","#bec936"),
        name_color_zh("huaihuahuanglv","槐花黄绿","#d2d97a"),
        name_color_zh("diehuang","蝶黄","#e2d849"),
        name_color_zh("xiangyabai","象牙白","#fffef8"),
        name_color_zh("ganlanlv","橄榄绿","#5e5314"),
        name_color_zh("xuebai","雪白","#fffef9"),
        name_color_zh("danhuilv","淡灰绿","#ad9e5f"),
        name_color_zh("foshouhuang","佛手黄","#fed71a"),
        name_color_zh("rubai","乳白","#f9f4dc"),
        name_color_zh("xiangjiaohuang","香蕉黄","#e4bf11"),
        name_color_zh("xinhelv","新禾绿","#d2b116"),
        name_color_zh("youcaihuahuang","油菜花黄","#fbda41"),
        name_color_zh("qiukuihuang","秋葵黄","#eed045"),
        name_color_zh("youhuang","柚黄","#f1ca17"),
        name_color_zh("caohuang","草黄","#d2b42c"),
        name_color_zh("liuhuahuang","硫华黄","#f2ce2b"),
        name_color_zh("jianghuang","姜黄","#e2c027"),
        name_color_zh("tanshuilv","潭水绿","#645822"),
        name_color_zh("jinguahuang","金瓜黄","#fcd217"),
        name_color_zh("maiganhuang","麦秆黄","#f8df70"),
        name_color_zh("haohuang","蒿黄","#dfc243"),
        name_color_zh("molihuang","茉莉黄","#f8df72"),
        name_color_zh("tenghuang","藤黄","#ffd111"),
        name_color_zh("mangguohuang","芒果黄","#ddc871"),
        name_color_zh("haishenhui","海参灰","#fffefa"),
        name_color_zh("biluochunlv","碧螺春绿","#867018"),
        name_color_zh("tailv","苔绿","#887322"),
        name_color_zh("ningmenghuang","柠檬黄","#fcd337"),
        name_color_zh("caohuilv","草灰绿","#8e804b"),
        name_color_zh("xiangrikuihuang","向日葵黄","#fecc11"),
        name_color_zh("suxinhuang","素馨黄","#fccb16"),
        name_color_zh("ruyahuang","乳鸭黄","#ffc90c"),
        name_color_zh("yuehui","月灰","#b7ae8f"),
        name_color_zh("kuishanhuang","葵扇黄","#f8d86a"),
        name_color_zh("dadouhuang","大豆黄","#fbcd31"),
        name_color_zh("jinzhanhuang","金盏黄","#fcc307"),
        name_color_zh("juleibai","菊蕾白","#e9ddb6"),
        name_color_zh("huanglianhuang","黄连黄","#fcc515"),
        name_color_zh("xingrenhuang","杏仁黄","#f7e8aa"),
        name_color_zh("guhuang","谷黄","#e8b004"),
        name_color_zh("muguahuang","木瓜黄","#f9c116"),
        name_color_zh("danjianhuang","淡茧黄","#f9d770"),
        name_color_zh("yalihuang","雅梨黄","#fbc82f"),
        name_color_zh("yinbai","银白","#f1f0ed"),
        name_color_zh("zonglvlv","棕榈绿","#5b4913"),
        name_color_zh("yingwuguanhuang","鹦鹉冠黄","#f6c430"),
        name_color_zh("kulv","枯绿","#b78d12"),
        name_color_zh("qianlaohuang","浅烙黄","#f9bd10"),
        name_color_zh("danmihuang","淡密黄","#f9d367"),
        name_color_zh("jiehuang","芥黄","#d9a40e"),
        name_color_zh("zhizihuang","栀子黄","#ebb10d"),
        name_color_zh("anhaishuilv","暗海水绿","#584717"),
        name_color_zh("miehuang","篾黄","#f7de98"),
        name_color_zh("bangroubai","蚌肉白","#f9f1db"),
        name_color_zh("chaomihuang","炒米黄","#f4ce69"),
        name_color_zh("hupohuang","琥珀黄","#feba07"),
        name_color_zh("huilv","灰绿","#8a6913"),
        name_color_zh("zongyelv","粽叶绿","#876818"),
        name_color_zh("chenhui","尘灰","#b6a476"),
        name_color_zh("youhuang","鼬黄","#fcb70a"),
        name_color_zh("xiangyahuang","象牙黄","#f0d695"),
        name_color_zh("jiaoqing","鲛青","#87723e"),
        name_color_zh("douzhihuang","豆汁黄","#f8e8c1"),
        name_color_zh("tuhuang","土黄","#d6a01d"),
        name_color_zh("xiangshuimeiguihuang","香水玫瑰黄","#f7da94"),
        name_color_zh("hupihuang","虎皮黄","#eaad1a"),
        name_color_zh("jidanhuang","鸡蛋黄","#fbb612"),
        name_color_zh("yinshuhui","银鼠灰","#b5aa90"),
        name_color_zh("yudubai","鱼肚白","#f7f4ed"),
        name_color_zh("chushuxinghuang","初熟杏黄","#f8bc31"),
        name_color_zh("shanjihuang","山鸡黄","#b78b26"),
        name_color_zh("lianzibai","莲子白","#e5d3aa"),
        name_color_zh("xiekehui","蟹壳灰","#695e45"),
        name_color_zh("shashihuang","沙石黄","#e5b751"),
        name_color_zh("gancaohuang","甘草黄","#f3bf4c"),
        name_color_zh("yanyuhui","燕羽灰","#685e48"),
        name_color_zh("ezhanghuang","鹅掌黄","#fbb929"),
        name_color_zh("maiyatanghuang","麦芽糖黄","#f9d27d"),
        name_color_zh("qiantuose","浅驼色","#e2c17c"),
        name_color_zh("bailingniaohui","百灵鸟灰","#b4a992"),
        name_color_zh("laohuang","酪黄","#f6dead"),
        name_color_zh("liroubai","荔肉白","#f2e6ce"),
        name_color_zh("danrouse","淡肉色","#f8e0b0"),
        name_color_zh("hetunhui","河豚灰","#393733"),
        name_color_zh("yililv","蜴蜊绿","#835e1d"),
        name_color_zh("hanbaiyu","汉白玉","#f8f4ed"),
        name_color_zh("chengpihuang","橙皮黄","#fca104"),
        name_color_zh("laiyanglihuang","莱阳梨黄","#815f25"),
        name_color_zh("pipahuang","枇杷黄","#fca106"),
        name_color_zh("jinyehuang","金叶黄","#ffa60f"),
        name_color_zh("canghuang","苍黄","#806332"),
        name_color_zh("fenbai","粉白","#fbf2e3"),
        name_color_zh("danjucheng","淡橘橙","#fba414"),
        name_color_zh("zhenzhuhui","珍珠灰","#e4dfd7"),
        name_color_zh("guibeihuang","龟背黄","#826b48"),
        name_color_zh("qianhui","浅灰","#dad4cb"),
        name_color_zh("zhonghui","中灰","#bbb5ac"),
        name_color_zh("qianhui","铅灰","#bbb5ac"),
        name_color_zh("xionghuang","雄黄","#ff9900"),
        name_color_zh("mihuang","蜜黄","#fbb957"),
        name_color_zh("fengfanhuang","风帆黄","#dc9123"),
        name_color_zh("guipidanzong","桂皮淡棕","#c09351"),
        name_color_zh("jinyinghuang","金莺黄","#f4a83a"),
        name_color_zh("rouse","肉色","#f7c173"),
        name_color_zh("diaoyezong","凋叶棕","#e7a23f"),
        name_color_zh("gutonglv","古铜绿","#533c1b"),
        name_color_zh("luoyingdanfen","落英淡粉","#f9e8d0"),
        name_color_zh("ruanmuhuang","软木黄","#de9e44"),
        name_color_zh("guarangfen","瓜瓤粉","#f9cb8b"),
        name_color_zh("liuehuang","榴萼黄","#f9a633"),
        name_color_zh("daimaohuang","玳瑁黄","#daa45a"),
        name_color_zh("jiaochalv","焦茶绿","#553b18"),
        name_color_zh("xiekelv","蟹壳绿","#513c20"),
        name_color_zh("shanjihe","山鸡褐","#986524"),
        name_color_zh("houmaohui","猴毛灰","#97846c"),
        name_color_zh("lujiaozong","鹿角棕","#e3bd8d"),
        name_color_zh("dansongyan","淡松烟","#4d4030"),
        name_color_zh("wanshoujuhuang","万寿菊黄","#fb8b05"),
        name_color_zh("dankehuang","蛋壳黄","#f8c387"),
        name_color_zh("xinghuang","杏黄","#f28e16"),
        name_color_zh("ganlanhui","橄榄灰","#503e2a"),
        name_color_zh("hehui","鹤灰","#4a4035"),
        name_color_zh("manaohui","玛瑙灰","#cfccc9"),
        name_color_zh("danyinhui","淡银灰","#c1b2a3"),
        name_color_zh("wahui","瓦灰","#867e76"),
        name_color_zh("yehui","夜灰","#847c74"),
        name_color_zh("beiguahuang","北瓜黄","#fc8c23"),
        name_color_zh("hehuabai","荷花白","#fbecde"),
        name_color_zh("songshuhui","松鼠灰","#4f4032"),
        name_color_zh("danmifen","淡米粉","#fbeee2"),
        name_color_zh("shenhui","深灰","#81776e"),
        name_color_zh("haiouhui","海鸥灰","#9a8878"),
        name_color_zh("chahe","茶褐","#5d3d21"),
        name_color_zh("tuose","驼色","#66462a"),
        name_color_zh("yinhui","银灰","#918072"),
        name_color_zh("lupihe","鹿皮褐","#d99156"),
        name_color_zh("binglangzong","槟榔综","#c1651a"),
        name_color_zh("xiaohui","晓灰","#d4c4b7"),
        name_color_zh("danzhe","淡赭","#be7e4a"),
        name_color_zh("gutonghe","古铜褐","#5c3719"),
        name_color_zh("jizong","麂棕","#de7622"),
        name_color_zh("zuiguarou","醉瓜肉","#db8540"),
        name_color_zh("yanhui","雁灰","#80766e"),
        name_color_zh("guiyuhong","鲑鱼红","#f09c5a"),
        name_color_zh("jucheng","橘橙","#f97d1c"),
        name_color_zh("jinhuang","金黄","#f26b1f"),
        name_color_zh("meiguifen","玫瑰粉","#f8b37f"),
        name_color_zh("meirenjiaocheng","美人焦橙","#fa7e23"),
        name_color_zh("mise","米色","#f9e9cd"),
        name_color_zh("zhuwanghui","蛛网灰","#b7a091"),
        name_color_zh("dankafei","淡咖啡","#945833"),
        name_color_zh("hailuocheng","海螺橙","#f0945d"),
        name_color_zh("yanshizong","岩石棕","#964d22"),
        name_color_zh("mangguozong","芒果棕","#954416"),
        name_color_zh("taocihong","陶瓷红","#e16723"),
        name_color_zh("boluohong","菠萝红","#fc7930"),
        name_color_zh("yujinhong","余烬红","#cf7543"),
        name_color_zh("jinlianhuacheng","金莲花橙","#f86b1d"),
        name_color_zh("huozhuanhong","火砖红","#cd6227"),
        name_color_zh("chutaofenhong","初桃粉红","#f6dcce"),
        name_color_zh("tiezong","铁棕","#d85916"),
        name_color_zh("jieqiaodanfenhong","介壳淡粉红","#f7cfba"),
        name_color_zh("xiekehong","蟹壳红","#f27635"),
        name_color_zh("jintuo","金驼","#e46828"),
        name_color_zh("yanhanhong","燕颔红","#fc6315"),
        name_color_zh("dankekezong","淡可可棕","#b7511d"),
        name_color_zh("chenxihong","晨曦红","#ea8958"),
        name_color_zh("yufenhong","玉粉红","#e8b49a"),
        name_color_zh("yeqiangweihong","野蔷薇红","#fb9968"),
        name_color_zh("ouhe","藕荷","#edc3ae"),
        name_color_zh("changshihui","长石灰","#363433"),
        name_color_zh("zhonghonghui","中红灰","#8b614d"),
        name_color_zh("huonizong","火泥棕","#aa6a4c"),
        name_color_zh("ganhong","绀红","#aa6a4c"),
        name_color_zh("meijianghong","莓酱红","#fa5d19"),
        name_color_zh("dingxiangzong","丁香棕","#71361d"),
        name_color_zh("danmeiguihui","淡玫瑰灰","#b89485"),
        name_color_zh("guaranghong","瓜瓤红","#f68c60"),
        name_color_zh("dancanghuahong","淡藏花红","#f6ad8f"),
        name_color_zh("sunpizong","筍皮棕","#732e12"),
        name_color_zh("runhong","润红","#f7cdbc"),
        name_color_zh("longjingyuhong","龙睛鱼红","#ef632b"),
        name_color_zh("dantuhuang","淡土黄","#8c4b31"),
        name_color_zh("zhumuhui","珠母灰","#64483d"),
        name_color_zh("furonghong","芙蓉红","#f9723d"),
        name_color_zh("luoxiahong","落霞红","#cf4813"),
        name_color_zh("faluohong","法螺红","#ee8055"),
        name_color_zh("caozhuhong","草珠红","#f8ebe6"),
        name_color_zh("kafei","咖啡","#753117"),
        name_color_zh("zhonghuituo","中灰驼","#603d30"),
        name_color_zh("yekezong","椰壳棕","#883a1e"),
        name_color_zh("xiemaohong","蟹蝥红","#b14b28"),
        name_color_zh("dandousha","淡豆沙","#873d24"),
        name_color_zh("dantaohong","淡桃红","#f6cec1"),
        name_color_zh("dantiehui","淡铁灰","#5b423a"),
        name_color_zh("shibanhui","石板灰","#624941"),
        name_color_zh("danlizong","淡栗棕","#673424"),
        name_color_zh("yinzhu","银朱","#f43e06"),
        name_color_zh("caomeihong","草莓红","#ef6f48"),
        name_color_zh("yangshuixianhong","洋水仙红","#f4c7ba"),
        name_color_zh("zhuhong","朱红","#ed5126"),
        name_color_zh("liuhuahong","榴花红","#f34718"),
        name_color_zh("shihong","柿红","#f2481b"),
        name_color_zh("kekezong","可可棕","#652b1c"),
        name_color_zh("danyingsuhong","淡罂粟红","#eea08c"),
        name_color_zh("dahong","大红","#f04b22"),
        name_color_zh("zhayezong","柞叶棕","#692a1b"),
        name_color_zh("qingtinghong","蜻蜓红","#f1441d"),
        name_color_zh("xiangshuzong","橡树棕","#773d31"),
        name_color_zh("jiahong","颊红","#eeaa9c"),
        name_color_zh("taohong","桃红","#f0ada0"),
        name_color_zh("huoyanzong","火岩棕","#863020"),
        name_color_zh("dantengluozi","淡藤萝紫","#f2e7e5"),
        name_color_zh("zheshi","赭石","#862617"),
        name_color_zh("tieshuihong","铁水红","#f5391c"),
        name_color_zh("yanzhihong","胭脂红","#f03f24"),
        name_color_zh("jiguanghong","极光红","#f33b1f"),
        name_color_zh("honggonghong","红汞红","#f23e23"),
        name_color_zh("luobohong","萝卜红","#f13c22"),
        name_color_zh("quhong","曲红","#f05a46"),
        name_color_zh("guqiaohong","谷鞘红","#f17666"),
        name_color_zh("pingguohong","苹果红","#f15642"),
        name_color_zh("guihong","桂红","#f25a47"),
        name_color_zh("fenhong","粉红","#f2b9b2"),
        name_color_zh("antuozong","暗驼棕","#592620"),
        name_color_zh("xiyanghong","夕阳红","#de2a18"),
        name_color_zh("yingtaohong","樱桃红","#ed3321"),
        name_color_zh("shanhuhong","珊瑚红","#f04a3a"),
        name_color_zh("huoshanzong","火山棕","#482522"),
        name_color_zh("lizong","栗棕","#5c1e19"),
        name_color_zh("hedinghong","鹤顶红","#d42517"),
        name_color_zh("shehong","舌红","#f19790"),
        name_color_zh("exueshihong","鹅血石红","#ab372f"),
        name_color_zh("jiangzong","酱棕","#5a1f1b"),
        name_color_zh("yusaihong","鱼鳃红","#ed3b2f"),
        name_color_zh("lusuihui","芦穗灰","#bdaead"),
        name_color_zh("lichunhong","丽春红","#eb261a"),
        name_color_zh("fupenzihong","覆盆子红","#ac1f18"),
        name_color_zh("haibaohui","海报灰","#483332"),
        name_color_zh("dousha","豆沙","#481e1c"),
        name_color_zh("liuzihong","榴子红","#f1908c"),
        name_color_zh("qiuhaitanghong","秋海棠红","#ec2b24"),
        name_color_zh("wuhuaguohong","无花果红","#efafad"),
        name_color_zh("danfei","淡绯","#f2cac9"),
        name_color_zh("meiguihui","玫瑰灰","#4b2e2b"),
        name_color_zh("danshuhong","淡菽红","#ed4845"),
        name_color_zh("goushuhong","枸枢红","#ed3333"),
        name_color_zh("diaozi","貂紫","#5d3131"),
    ];
}

// 李紫 lizi #2b1216

#[test]
fn test_name_color_zh() {
    let nz = NamedColorZh{
        name:"lizi",
        name_zh:"李紫",
        color: Color::from_rgb(43,18,22),
//        hex_color:"2b1216",
    };
    assert_eq!(nz, name_color_zh("lizi","李紫","#2b1216"));
}