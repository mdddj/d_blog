import React, { useEffect, useState } from 'react';
import { PageContainer } from '@ant-design/pro-components';
import { Button, Card, Col, Form, Input, Row, Space } from 'antd';
import { CloseOutlined } from '@ant-design/icons';

const { Item } = Form;

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

//产品管理页面
export default function Page() {
  const [form] = Form.useForm();
  const skuList = Form.useWatch('skuList', form) as SkuItem[];
  console.log(skuList);
  let skuMap: SkuPriceItem[] = [];
  if (skuList && skuList.length !== 0) {
    skuMap = generateSkuList(skuList);
  }
  console.log(skuMap);

  function onTestValue() {

    form.setFieldValue('skuList', [
      {
        'name': '颜色',
        'list': [
          { 'value': '蓝色' },
          { 'value': '红色' },
          { 'value': '黑色' },
        ],
      },
      {
        'name': '尺码',
        'list': [
          { 'value': 'xl' },
          { 'value': 'xxl' },
          { 'value': 'l' },
        ],
      },
    ]);

  }

  console.log('re builder ...');
  return (
    <PageContainer>
      <Row gutter={12}>
        <Col span={12}>
          <Button onClick={onTestValue}>测试</Button>
          <Form form={form}>
            <Form.List name="skuList">
              {(fields, { add, remove }) => (
                <div style={{ display: 'flex', rowGap: 16, flexDirection: 'column' }}>
                  {fields.map((field) => (
                    <Card
                      size="small"
                      title={`产品属性 ${field.name + 1}`}
                      key={field.key}
                      extra={
                        <CloseOutlined
                          onClick={() => {
                            remove(field.name);
                          }}
                        />
                      }
                    >
                      <Form.Item label="属性名称" name={[field.name, 'name']}>
                        <Input />
                      </Form.Item>

                      {/* Nest Form.List */}
                      <Form.Item label="Sku">
                        <Form.List name={[field.name, 'list']}>
                          {(subFields, subOpt) => (
                            <div style={{ display: 'flex', flexDirection: 'column', rowGap: 16 }}>
                              {subFields.map((subField) => (
                                <Space key={subField.key}>
                                  <Form.Item noStyle name={[subField.name, 'value']}>
                                    <Input placeholder="输入sku名称" />
                                  </Form.Item>
                                  <CloseOutlined
                                    onClick={() => {
                                      subOpt.remove(subField.name);
                                    }}
                                  />
                                </Space>
                              ))}
                              <Button type="dashed" onClick={() => subOpt.add()} block>
                                + 添加SKU
                              </Button>
                            </div>
                          )}
                        </Form.List>
                      </Form.Item>
                    </Card>
                  ))}

                  <Button type="dashed" onClick={() => add()} block>
                    + 添加属性
                  </Button>
                </div>
              )}
            </Form.List>
          </Form>
        </Col>
        <Col span={12}>
          <SkuForm skuMap={skuMap} />
        </Col>
      </Row>

    </PageContainer>
  );
}

///设置库存和价格
const SkuForm: React.FC<{ skuMap: SkuPriceItem[] }> = ({ skuMap }) => {
  const [formData, setFormData] = useState<SkuPriceItem[]>([]);

  useEffect(() => {
    setFormData(skuMap);
  });


  const handleChange = (index: number, field: keyof SkuPriceItem, value: string) => {
    console.log('change' + index + 'field: ' + field + '  value: ' + value);
    setFormData(prevData => {
      return prevData.map((item, i) => {
        if (i !== index) return item;
        return { ...item, [field]: value };
      });
    });
  };

  //
  const handleSubmit = () => {
    // 处理表单提交逻辑，将数据发送给后端等
    console.log('Form data:', formData);
  };
  console.log(formData);
  return (
    <Card title={'设置库存和价格'}>
      {formData.map((value, index) => {
        const skuName = value.skuValues.map(value1 => value1.value).join(',');
        return (
          <Card key={skuName} size={'small'} className={'mt-2'}>
            <span className={'text-fuchsia-950 font-bold text-2xl'}>{skuName}</span>
            <Form layout="inline" className={'mt-5'}>
              <Item label="价格">
                <Input
                  value={formData[index] ? formData[index].price : ''}
                  onChange={e => handleChange(index, 'price', e.target.value)}
                />
              </Item>
              <Item label="库存">
                <Input
                  value={formData[index] ? formData[index].stock : ''}
                  onChange={e => handleChange(index, 'stock', e.target.value)}
                />
              </Item>
            </Form>
          </Card>
        );
      })}
      <Button type="primary" onClick={handleSubmit}>提交</Button>
    </Card>
  );
};