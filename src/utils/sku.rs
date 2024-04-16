
use std::collections::{HashMap, HashSet};
use crate::dtos::product_sku::SkuCombination;
use crate::dtos::product_sku_value::ProductSkuNameVecValueModel;

pub fn list_all_possible_skus(models: &[ProductSkuNameVecValueModel]) -> Vec<SkuCombination> {
    // 创建一个 HashMap 用于存储每个 SKU 名称的可能值
    let mut sku_values_map: HashMap<String, HashSet<String>> = HashMap::new();

    // 遍历每个 ProductSkuNameVecValueModel
    for model in models {
        // 获取对应的 HashSet，如果没有则插入新的空 HashSet
        let values_set = sku_values_map.entry(model.name.clone()).or_insert_with(HashSet::new);

        // 将每个 SKU 值添加到对应的 HashSet 中
        for value in &model.values {
            values_set.insert(value.clone());
        }
    }

    // 组合所有可能的 SKU 组合
    let mut result: Vec<SkuCombination> = Vec::new();

    // 使用递归函数组合所有可能的 SKU 组合
    fn combine_skus(
        sku_values_map: &HashMap<String, HashSet<String>>,
        current_index: usize,
        current_combination: SkuCombination,
        result: &mut Vec<SkuCombination>,
    ) {
        if current_index >= sku_values_map.len() {
            result.push(current_combination);
            return;
        }

        let (sku_name, sku_values_set) = sku_values_map.iter().nth(current_index).unwrap();

        for value in sku_values_set {
            let mut new_combination = current_combination.clone();
            new_combination.combinations.push((sku_name.clone(), value.clone()));
            combine_skus(sku_values_map, current_index + 1, new_combination, result);
        }
    }

    combine_skus(&sku_values_map, 0, SkuCombination{ combinations: Vec::new() }, &mut result);

    result
}
