function toRustValue (value) {
  if (value === undefined) {
    throw new Error("Undefined value found. This likely means a new enum variant lookup needs to be added");
  }

  var n = value.replace('px', '').replace('%', '')

  if (value.includes('px')) {
    return 'StyleUnit::Point(' + n + '_f32.into())'
  }

  if (value.includes('%')) {
    return 'StyleUnit::Percent(' + n + '_f32.into())'
  }

  // Handle aspect ratio values in "1 / 3" form
  if (value.includes('/')) {
    const [numerator, denominator] = value.split('/');
    return toRustNumber(numerator) + ' / ' + toRustNumber(denominator);
  }

  var number = Number(n)
  if (isNaN(number)) {
    return value
  }
  return number + '_f32'
}

function toRustNumber (value) {
  var valueString = value.toString()
  var n = value.toString().replace('px', '').replace('%', '')
  var number = Number(n)
  if (isNaN(number)) {
    return valueString
  }
  return number + '_f32'
}

var RustEmitter = function () {
  Emitter.call(this, 'rs', '\t')
}

RustEmitter.prototype = Object.create(Emitter.prototype, {

  constructor: { value: RustEmitter },

  emitPrologue: {
    value: function () {
      this.push([
        'extern crate ordered_float;',
        'extern crate yoga;',
        'mod test_utils;',
        '',
        'use yoga::*;',
        ''
      ])
    }
  },

  emitEpilogue: {
    value: function (lines) {

    }
  },

  emitTestPrologue: {
    value: function (name, experiments, disabled) {
      this.push('#[test]');
      if (disabled) {
        this.push('#[ignore]');
      }
      this.push('fn test_' + name + '() {');
      this.pushIndent();
      this.push([
        'let mut config = Config::new();',
        ''
      ]);
    }
  },

  emitTestTreePrologue: {
    value: function (nodeName) {
      this.push([
        'let mut ' + nodeName + ' = Node::new_with_config(&mut config);'
      ])
    }
  },

  emitTestEpilogue: {
    value: function (experiments) {
      this.popIndent()
      this.push([
        '}',
        ''
      ])
    }
  },

  AssertEQ: {
    value: function (v0, v1) {
      this.push([
        'assert_eq!(' + v0 + '_f32, ' + v1 + ');'
      ])
    }
  },

  YGAlignAuto: { value: 'Align::Auto' },
  YGAlignCenter: { value: 'Align::Center' },
  YGAlignFlexEnd: { value: 'Align::FlexEnd' },
  YGAlignFlexStart: { value: 'Align::FlexStart' },
  YGAlignStretch: { value: 'Align::Stretch' },
  YGAlignSpaceBetween: { value: 'Align::SpaceBetween' },
  YGAlignSpaceAround: { value: 'Align::SpaceAround' },
  YGAlignSpaceEvenly: { value: 'Align::SpaceEvenly' },
  YGAlignBaseline: { value: 'Align::Baseline' },

  YGDirectionInherit: { value: 'Direction::Inherit' },
  YGDirectionLTR: { value: 'Direction::LTR' },
  YGDirectionRTL: { value: 'Direction::RTL' },

  YGEdgeBottom: { value: 'Edge::Bottom' },
  YGEdgeEnd: { value: 'Edge::End' },
  YGEdgeLeft: { value: 'Edge::Left' },
  YGEdgeRight: { value: 'Edge::Right' },
  YGEdgeStart: { value: 'Edge::Start' },
  YGEdgeTop: { value: 'Edge::Top' },

  YGGutterAll:{value:'Gutter::All'},
  YGGutterColumn:{value:'Gutter::Column'},
  YGGutterRow:{value:'Gutter::Row'},

  YGFlexDirectionColumn: { value: 'FlexDirection::Column' },
  YGFlexDirectionColumnReverse: { value: 'FlexDirection::ColumnReverse' },
  YGFlexDirectionRow: { value: 'FlexDirection::Row' },
  YGFlexDirectionRowReverse: { value: 'FlexDirection::RowReverse' },

  YGJustifyCenter: { value: 'Justify::Center' },
  YGJustifyFlexEnd: { value: 'Justify::FlexEnd' },
  YGJustifyFlexStart: { value: 'Justify::FlexStart' },
  YGJustifySpaceAround: { value: 'Justify::SpaceAround' },
  YGJustifySpaceBetween: { value: 'Justify::SpaceBetween' },
  YGJustifySpaceEvenly: { value: 'Justify::SpaceEvenly' },

  YGOverflowHidden: { value: 'Overflow::Hidden' },
  YGOverflowVisible: { value: 'Overflow::Visible' },
  YGOverflowScroll: { value: 'Overflow::Scroll' },

  YGPositionTypeAbsolute: { value: 'PositionType::Absolute' },
  YGPositionTypeRelative: { value: 'PositionType::Relative' },
  YGPositionTypeStatic: { value: 'PositionType::Static' },

  YGUndefined: { value: 'StyleUnit::UndefinedValue' },

  YGAuto: { value: 'StyleUnit::Auto' },

  YGDisplayFlex: { value: 'Display::Flex' },
  YGDisplayNone: { value: 'Display::None' },
  YGDisplayContents: { value: 'Display::Contents' },

  YGWrapNoWrap: { value: 'Wrap::NoWrap' },
  YGWrapWrap: { value: 'Wrap::Wrap' },
  YGWrapWrapReverse: { value: 'Wrap::WrapReverse' },

  YGBoxSizingBorderBox: {value: 'BoxSizing::BorderBox'},
  YGBoxSizingContentBox: {value: 'BoxSizing::ContentBox'},

  YGNodeCalculateLayout: {
    value: function (node, dir, experiments) {
      this.push(node + '.calculate_layout(Undefined, Undefined, ' + dir + ');')
    }
  },

  YGNodeInsertChild: {
    value: function (parentName, nodeName, index) {
      this.push(parentName + '.insert_child(&mut ' + nodeName + ', ' + index + ');')
    }
  },

  YGNodeLayoutGetLeft: {
    value: function (nodeName) {
      return nodeName + '.get_layout_left()'
    }
  },

  YGNodeLayoutGetTop: {
    value: function (nodeName) {
      return nodeName + '.get_layout_top()'
    }
  },

  YGNodeLayoutGetWidth: {
    value: function (nodeName) {
      return nodeName + '.get_layout_width()'
    }
  },

  YGNodeLayoutGetHeight: {
    value: function (nodeName) {
      return nodeName + '.get_layout_height()'
    }
  },

  YGNodeStyleSetAlignContent: {
    value: function (nodeName, value) {
      this.push(nodeName + '.set_align_content(' + value + ');')
    }
  },

  YGNodeStyleSetAlignItems: {
    value: function (nodeName, value) {
      this.push(nodeName + '.set_align_items(' + value + ');')
    }
  },

  YGNodeStyleSetAlignSelf: {
    value: function (nodeName, value) {
      this.push(nodeName + '.set_align_self(' + value + ');')
    }
  },

  YGNodeStyleSetAspectRatio: {
    value: function (nodeName, value) {
      this.push(nodeName + '.set_aspect_ratio(' + toRustValue(value) + ');');
    },
  },

  YGNodeStyleSetBorder: {
    value: function (nodeName, edge, value) {
      var strippedValue = value.toString().replace('px', '')
      this.push(nodeName + '.set_border(' + edge + ', ' + (strippedValue) + '_f32);')
    }
  },

  YGNodeStyleSetDirection: {
    value: function (nodeName, value) {
      this.push(nodeName + '.set_direction(' + value + ');')
    }
  },

  YGNodeStyleSetDisplay: {
    value: function (nodeName, value) {
      this.push(nodeName + '.set_display(' + value + ');')
    }
  },

  YGNodeStyleSetFlexBasis: {
    value: function (nodeName, value) {
      this.push(nodeName + '.set_flex_basis(' + toRustValue(value) + ');')
    }
  },

  YGNodeStyleSetFlexDirection: {
    value: function (nodeName, value) {
      this.push(nodeName + '.set_flex_direction(' + value + ');')
    }
  },

  YGNodeStyleSetFlexGrow: {
    value: function (nodeName, value) {
      this.push(nodeName + '.set_flex_grow(' + toRustValue(value) + ');')
    }
  },

  YGNodeStyleSetFlexShrink: {
    value: function (nodeName, value) {
      this.push(nodeName + '.set_flex_shrink(' + toRustValue(value) + ');')
    }
  },

  YGNodeStyleSetFlexWrap: {
    value: function (nodeName, value) {
      this.push(nodeName + '.set_flex_wrap(' + value + ');')
    }
  },

  YGNodeStyleSetHeight: {
    value: function (nodeName, value) {
      this.push(nodeName + '.set_height(' + toRustValue(value) + ');')
    }
  },

  YGNodeStyleSetJustifyContent: {
    value: function (nodeName, value) {
      this.push(nodeName + '.set_justify_content(' + value + ');')
    }
  },

  YGNodeStyleSetMargin: {
    value: function (nodeName, edge, value) {
      this.push(nodeName + '.set_margin(' + edge + ', ' + toRustValue(value) + ');')
    }
  },

  YGNodeStyleSetMaxHeight: {
    value: function (nodeName, value) {
      this.push(nodeName + '.set_max_height(' + toRustValue(value) + ');')
    }
  },

  YGNodeStyleSetMaxWidth: {
    value: function (nodeName, value) {
      this.push(nodeName + '.set_max_width(' + toRustValue(value) + ');')
    }
  },

  YGNodeStyleSetMinHeight: {
    value: function (nodeName, value) {
      this.push(nodeName + '.set_min_height(' + toRustValue(value) + ');')
    }
  },

  YGNodeStyleSetMinWidth: {
    value: function (nodeName, value) {
      this.push(nodeName + '.set_min_width(' + toRustValue(value) + ');')
    }
  },

  YGNodeStyleSetOverflow: {
    value: function (nodeName, value) {
      this.push(nodeName + '.set_overflow(' + toRustValue(value) + ');')
    }
  },

  YGNodeStyleSetPadding: {
    value: function (nodeName, edge, value) {
      this.push(nodeName + '.set_padding(' + edge + ', ' + toRustValue(value) + ');')
    }
  },

  YGNodeStyleSetPosition: {
    value: function (nodeName, edge, value) {
      this.push(nodeName + '.set_position(' + edge + ', ' + toRustValue(value) + ');')
    }
  },

  YGNodeStyleSetPositionType: {
    value: function (nodeName, value) {
      this.push(nodeName + '.set_position_type(' + toRustValue(value) + ');')
    }
  },

  YGNodeStyleSetWidth: {
    value: function (nodeName, value) {
      this.push(nodeName + '.set_width(' + toRustValue(value) + ');')
    }
  },

  YGNodeStyleSetGap:{value:function(nodeName, gap, value) {
    this.push(nodeName + '.set_gap('+ gap + ', ' + toRustValue(value) + ');');
  }},

  YGNodeStyleSetBoxSizing: {
    value: function (nodeName, value) {
      this.push(nodeName + '.set_box_sizing(' + toRustValue(value) + ');');
    },
  },

  YGNodeSetMeasureFunc: {
    value: function (nodeName, innerText) {
      this.push(`${nodeName}.set_context(Some(Context::new(String::from("${innerText}"))));`);
      this.push(
        `${nodeName}.set_measure_func(Some(test_utils::intrinsic_measure_function));`,
      );
    },
  },
})
