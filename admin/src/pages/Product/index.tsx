

interface SkuValue {
  value: string;
}

interface SkuItem {
  name: string,
  list: SkuValue[]
}

interface SkuPriceItem {
  skuValues: SkuValue[],
  price: number,
  stock: number
}

function generateSkuList(skuList: SkuItem[]): SkuPriceItem[] {
  const skuOptions: SkuPriceItem[] = [];
  if (skuList.length === 0) {
    return skuOptions;
  }
  generateSkuOptions(skuList, 0, [], skuOptions);
  return skuOptions;
}

function generateSkuOptions(
  skuList: SkuItem[],
  index: number,
  currentOptions: SkuValue[],
  skuOptions: SkuPriceItem[],
) {
  if (index === skuList.length) {
    skuOptions.push({
      skuValues: [...currentOptions],
      price: 0, // 默认价格
      stock: 0, // 默认库存
    });
    return;
  }

  const option = skuList[index];
  if (option && option.list) {
    const values = option.list;
    for (const value of values) {
      if (value) {
        const optionValue = value.value;
        if (optionValue) {
          currentOptions.push({ value: optionValue });
          generateSkuOptions(skuList, index + 1, currentOptions, skuOptions);
          currentOptions.pop();
        }
      }
    }
  }

}
