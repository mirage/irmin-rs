#pragma once
#include <stdbool.h>
#include <stdint.h>
typedef struct IrminType IrminType;
typedef struct IrminValue IrminValue;
typedef struct IrminMetadata IrminMetadata;
typedef struct IrminContents IrminContents;
typedef struct IrminConfig IrminConfig;
typedef struct IrminRepo IrminRepo;
typedef struct Irmin Irmin;
typedef struct IrminPath IrminPath;
typedef struct IrminCommitKey IrminCommitKey;
typedef struct IrminKindedKey IrminKindedKey;
typedef struct IrminTree IrminTree;
typedef struct IrminCommit IrminCommit;
typedef struct IrminInfo IrminInfo;
typedef struct IrminHash IrminHash;
typedef struct IrminString IrminString;
typedef struct IrminPathArray IrminPathArray;
typedef struct IrminCommitArray IrminCommitArray;
typedef struct IrminBranchArray IrminBranchArray;
typedef struct IrminRemote IrminRemote;
void caml_startup(char *argv[]);
void caml_shutdown();
IrminType* irmin_type_unit(void);
IrminType* irmin_type_bool(void);
IrminType* irmin_type_int(void);
IrminType* irmin_type_float(void);
IrminType* irmin_type_string(void);
IrminType* irmin_type_bytes(void);
IrminType* irmin_type_list(IrminType* x1575);
IrminType* irmin_type_array(IrminType* x1576);
IrminType* irmin_type_option(IrminType* x1577);
IrminType* irmin_type_json(void);
IrminType* irmin_type_json_value(void);
IrminType* irmin_type_path(IrminRepo* x1580);
IrminType* irmin_type_commit(IrminRepo* x1581);
IrminType* irmin_type_metadata(IrminRepo* x1582);
IrminType* irmin_type_tree(IrminRepo* x1583);
IrminType* irmin_type_hash(IrminRepo* x1584);
IrminType* irmin_type_commit_key(IrminRepo* x1585);
IrminType* irmin_type_contents_key(IrminRepo* x1586);
IrminType* irmin_type_node_key(IrminRepo* x1587);
IrminType* irmin_type_kinded_key(IrminRepo* x1588);
IrminType* irmin_type_contents(IrminRepo* x1589);
IrminType* irmin_type_pair(IrminType* x1591, IrminType* x1590);
IrminType* irmin_type_triple(IrminType* x1594, IrminType* x1593,
                             IrminType* x1592);
IrminString* irmin_type_name(IrminType* x1595);
IrminType* irmin_type_diff(IrminType* x1596);
void irmin_type_free(IrminType* x1597);
IrminValue* irmin_value_unit(void);
IrminValue* irmin_value_int(int64_t x1599);
IrminValue* irmin_value_float(double x1600);
IrminValue* irmin_value_bool(_Bool x1601);
IrminValue* irmin_value_clone(IrminValue* x1602);
void* irmin_realloc(void* x1604, void* x1603);
IrminString* irmin_value_get_string(IrminValue* x1605);
int64_t irmin_value_get_int(IrminValue* x1606);
_Bool irmin_value_get_bool(IrminValue* x1607);
double irmin_value_get_float(IrminValue* x1608);
IrminValue* irmin_value_bytes(char* x1610, int64_t x1609);
IrminValue* irmin_value_string(char* x1612, int64_t x1611);
IrminValue* irmin_value_array(IrminValue** x1614, uint64_t x1613);
IrminValue* irmin_value_list(IrminValue** x1616, uint64_t x1615);
IrminValue* irmin_value_option(IrminValue* x1617);
IrminValue* irmin_value_pair(IrminValue* x1619, IrminValue* x1618);
IrminValue* irmin_value_triple(IrminValue* x1622, IrminValue* x1621,
                               IrminValue* x1620);
IrminString* irmin_value_to_string(IrminType* x1624, IrminValue* x1623);
IrminValue* irmin_value_of_string(IrminType* x1627, char* x1626,
                                  int64_t x1625);
IrminString* irmin_value_to_bin(IrminType* x1629, IrminValue* x1628);
IrminValue* irmin_value_of_bin(IrminType* x1632, char* x1631, int64_t x1630);
IrminString* irmin_value_to_json(IrminType* x1634, IrminValue* x1633);
IrminValue* irmin_value_of_json(IrminType* x1637, char* x1636, int64_t x1635);
_Bool irmin_value_equal(IrminType* x1640, IrminValue* x1639,
                        IrminValue* x1638);
int irmin_value_compare(IrminType* x1643, IrminValue* x1642,
                        IrminValue* x1641);
void irmin_value_free(IrminValue* x1644);
IrminString* irmin_string_new(char* x1646, int64_t x1645);
char* irmin_string_data(IrminString* x1647);
uint64_t irmin_string_length(IrminString* x1648);
void irmin_string_free(IrminString* x1649);
IrminInfo* irmin_info_new(IrminRepo* x1652, char* x1651, char* x1650);
void irmin_info_update(IrminRepo* x1656, IrminInfo* x1655, char* x1654,
                       char* x1653);
IrminString* irmin_info_message(IrminRepo* x1658, IrminInfo* x1657);
IrminString* irmin_info_author(IrminRepo* x1660, IrminInfo* x1659);
int64_t irmin_info_date(IrminRepo* x1662, IrminInfo* x1661);
void irmin_info_free(IrminInfo* x1663);
_Bool irmin_log_level(char* x1664);
IrminConfig* irmin_config_pack(char* x1666, char* x1665);
IrminConfig* irmin_config_tezos(void);
IrminConfig* irmin_config_git(char* x1668);
IrminConfig* irmin_config_git_mem(char* x1669);
IrminConfig* irmin_config_fs(char* x1671, char* x1670);
IrminConfig* irmin_config_mem(char* x1673, char* x1672);
void irmin_config_free(IrminConfig* x1674);
_Bool irmin_config_set(IrminConfig* x1678, char* x1677, IrminType* x1676,
                       IrminValue* x1675);
_Bool irmin_config_set_root(IrminConfig* x1680, char* x1679);
Irmin* irmin_main(IrminRepo* x1681);
Irmin* irmin_of_branch(IrminRepo* x1683, char* x1682);
Irmin* irmin_of_commit(IrminRepo* x1685, IrminCommit* x1684);
IrminCommit* irmin_get_head(Irmin* x1686);
void irmin_set_head(Irmin* x1688, IrminCommit* x1687);
_Bool irmin_fast_forward(Irmin* x1690, IrminCommit* x1689);
_Bool irmin_merge_with_branch(Irmin* x1693, char* x1692, IrminInfo* x1691);
_Bool irmin_merge_with_commit(Irmin* x1696, IrminCommit* x1695,
                              IrminInfo* x1694);
_Bool irmin_merge_into(Irmin* x1699, Irmin* x1698, IrminInfo* x1697);
_Bool irmin_set(Irmin* x1703, IrminPath* x1702, IrminContents* x1701,
                IrminInfo* x1700);
_Bool irmin_test_and_set(Irmin* x1708, IrminPath* x1707,
                         IrminContents* x1706, IrminContents* x1705,
                         IrminInfo* x1704);
_Bool irmin_test_and_set_tree(Irmin* x1713, IrminPath* x1712,
                              IrminTree* x1711, IrminTree* x1710,
                              IrminInfo* x1709);
_Bool irmin_set_tree(Irmin* x1717, IrminPath* x1716, IrminTree* x1715,
                     IrminInfo* x1714);
IrminContents* irmin_find(Irmin* x1719, IrminPath* x1718);
IrminMetadata* irmin_find_metadata(Irmin* x1721, IrminPath* x1720);
IrminTree* irmin_find_tree(Irmin* x1723, IrminPath* x1722);
_Bool irmin_remove(Irmin* x1726, IrminPath* x1725, IrminInfo* x1724);
_Bool irmin_mem(Irmin* x1728, IrminPath* x1727);
_Bool irmin_mem_tree(Irmin* x1730, IrminPath* x1729);
IrminPathArray* irmin_list(Irmin* x1732, IrminPath* x1731);
uint64_t irmin_path_array_length(IrminRepo* x1734, IrminPathArray* x1733);
IrminPath* irmin_path_array_get(IrminRepo* x1737, IrminPathArray* x1736,
                                uint64_t x1735);
IrminRemote* irmin_remote_store(Irmin* x1738);
IrminRemote* irmin_remote(IrminRepo* x1740, char* x1739);
IrminRemote* irmin_remote_with_auth(IrminRepo* x1744, char* x1743,
                                    char* x1742, char* x1741);
IrminCommit* irmin_fetch(Irmin* x1747, int x1746, IrminRemote* x1745);
IrminCommit* irmin_pull(Irmin* x1751, int x1750, IrminRemote* x1749,
                        IrminInfo* x1748);
IrminCommit* irmin_push(Irmin* x1754, int x1753, IrminRemote* x1752);
void irmin_remote_free(IrminRemote* x1755);
void irmin_path_array_free(IrminPathArray* x1756);
void irmin_free(Irmin* x1757);
IrminTree* irmin_tree_new(IrminRepo* x1758);
IrminTree* irmin_tree_of_contents(IrminRepo* x1761, IrminContents* x1760,
                                  IrminMetadata* x1759);
IrminTree* irmin_tree_clone(IrminRepo* x1763, IrminTree* x1762);
IrminHash* irmin_tree_hash(IrminRepo* x1765, IrminTree* x1764);
IrminTree* irmin_tree_of_hash(IrminRepo* x1767, IrminHash* x1766);
IrminKindedKey* irmin_tree_key(IrminRepo* x1769, IrminTree* x1768);
IrminTree* irmin_tree_of_key(IrminRepo* x1771, IrminKindedKey* x1770);
_Bool irmin_tree_mem(IrminRepo* x1774, IrminTree* x1773, IrminPath* x1772);
_Bool irmin_tree_mem_tree(IrminRepo* x1777, IrminTree* x1776,
                          IrminPath* x1775);
IrminContents* irmin_tree_find(IrminRepo* x1780, IrminTree* x1779,
                               IrminPath* x1778);
IrminMetadata* irmin_tree_find_metadata(IrminRepo* x1783, IrminTree* x1782,
                                        IrminPath* x1781);
IrminTree* irmin_tree_find_tree(IrminRepo* x1786, IrminTree* x1785,
                                IrminPath* x1784);
_Bool irmin_tree_add(IrminRepo* x1791, IrminTree* x1790, IrminPath* x1789,
                     IrminContents* x1788, IrminMetadata* x1787);
_Bool irmin_tree_add_tree(IrminRepo* x1795, IrminTree* x1794,
                          IrminPath* x1793, IrminTree* x1792);
_Bool irmin_tree_remove(IrminRepo* x1798, IrminTree* x1797, IrminPath* x1796);
_Bool irmin_tree_equal(IrminRepo* x1801, IrminTree* x1800, IrminTree* x1799);
IrminPathArray* irmin_tree_list(IrminRepo* x1804, IrminTree* x1803,
                                IrminPath* x1802);
_Bool irmin_kinded_key_is_contents(IrminRepo* x1806, IrminKindedKey* x1805);
_Bool irmin_kinded_key_is_node(IrminRepo* x1808, IrminKindedKey* x1807);
void irmin_tree_free(IrminTree* x1809);
void irmin_kinded_key_free(IrminKindedKey* x1810);
IrminRepo* irmin_repo_new(IrminConfig* x1811);
IrminBranchArray* irmin_repo_branches(IrminRepo* x1812);
uint64_t irmin_branch_array_length(IrminRepo* x1814, IrminBranchArray* x1813);
IrminString* irmin_branch_array_get(IrminRepo* x1817,
                                    IrminBranchArray* x1816, uint64_t x1815);
_Bool irmin_hash_equal(IrminRepo* x1820, IrminHash* x1819, IrminHash* x1818);
IrminHash* irmin_contents_hash(IrminRepo* x1822, IrminContents* x1821);
IrminContents* irmin_contents_of_hash(IrminRepo* x1824, IrminHash* x1823);
IrminContents* irmin_contents_of_key(IrminRepo* x1826, IrminKindedKey* x1825);
IrminString* irmin_contents_to_string(IrminRepo* x1828, IrminContents* x1827);
IrminContents* irmin_contents_of_string(IrminRepo* x1831, char* x1830,
                                        int64_t x1829);
IrminString* irmin_hash_to_string(IrminRepo* x1833, IrminHash* x1832);
IrminHash* irmin_hash_of_string(IrminRepo* x1836, char* x1835, int64_t x1834);
IrminMetadata* irmin_metadata_default(IrminRepo* x1837);
_Bool irmin_repo_has_error(IrminRepo* x1838);
IrminString* irmin_repo_get_error(IrminRepo* x1839);
void irmin_hash_free(IrminHash* x1840);
void irmin_branch_array_free(IrminBranchArray* x1841);
void irmin_repo_free(IrminRepo* x1842);
void irmin_metadata_free(IrminMetadata* x1843);
void irmin_contents_free(IrminContents* x1844);
IrminInfo* irmin_commit_info(IrminRepo* x1846, IrminCommit* x1845);
IrminHash* irmin_commit_hash(IrminRepo* x1848, IrminCommit* x1847);
IrminCommitKey* irmin_commit_key(IrminRepo* x1850, IrminCommit* x1849);
IrminCommit* irmin_commit_of_hash(IrminRepo* x1852, IrminHash* x1851);
IrminCommit* irmin_commit_of_key(IrminRepo* x1854, IrminCommitKey* x1853);
IrminCommit* irmin_commit_new(IrminRepo* x1859, IrminCommit** x1858,
                              uint64_t x1857, IrminTree* x1856,
                              IrminInfo* x1855);
IrminCommitArray* irmin_commit_parents(IrminRepo* x1861, IrminCommit* x1860);
_Bool irmin_commit_equal(IrminRepo* x1864, IrminCommit* x1863,
                         IrminCommit* x1862);
IrminTree* irmin_commit_tree(IrminRepo* x1866, IrminCommit* x1865);
uint64_t irmin_commit_array_length(IrminRepo* x1868, IrminCommitArray* x1867);
IrminCommit* irmin_commit_array_get(IrminRepo* x1871,
                                    IrminCommitArray* x1870, uint64_t x1869);
void irmin_commit_array_free(IrminCommitArray* x1872);
void irmin_commit_free(IrminCommit* x1873);
void irmin_commit_key_free(IrminCommitKey* x1874);
IrminPath* irmin_path(IrminRepo* x1876, char** x1875);
IrminPath* irmin_path_of_string(IrminRepo* x1879, char* x1878, int64_t x1877);
IrminPath* irmin_path_empty(IrminRepo* x1880);
IrminString* irmin_path_to_string(IrminRepo* x1882, IrminPath* x1881);
IrminPath* irmin_path_parent(IrminRepo* x1884, IrminPath* x1883);
IrminPath* irmin_path_append(IrminRepo* x1888, IrminPath* x1887, char* x1886,
                             int64_t x1885);
IrminPath* irmin_path_append_path(IrminRepo* x1891, IrminPath* x1890,
                                  IrminPath* x1889);
_Bool irmin_path_equal(IrminRepo* x1894, IrminPath* x1893, IrminPath* x1892);
void irmin_path_free(IrminPath* x1895);


#ifndef IRMIN_NO_AUTO
static void _irmin_cleanup(void *p) { if (p) { irmin_free(*(Irmin**)p); p = (void*)0;} };
#define AUTO __attribute__((cleanup(_irmin_cleanup)))
#endif
    
