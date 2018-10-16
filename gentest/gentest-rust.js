function toRustValue (value) {
  var valueString = value.toString()
  var n = value.toString().replace('px', '').replace('%', '')

  if (valueString.includes('px')) {
    return 'StyleUnit::Point((' + n + ' as f32).into())'
  }

  if (valueString.includes('%')) {
    return 'StyleUnit::Percent((' + n + ' as f32).into())'
  }

  var number = Number(n)
  if (isNaN(number)) {
    return valueString
  }
  return number + ' as f32'
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
    value: function (name, experiments) {
      this.push([
        '#[test]',
        'fn test_' + name + '() {'
      ])
      this.pushIndent()
      this.push([
        'let mut config = Config::new();',
        ''
      ])
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
        'assert_eq!(' + v0 + ' as f32, ' + v1 + ');'
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

  YGPositionTypeAbsolute: { value: 'PositionType::Absolute' },
  YGPositionTypeRelative: { value: 'PositionType::Relative' },

  YGUndefined: { value: 'StyleUnit::UndefinedValue' },

  YGAuto: { value: 'StyleUnit::Auto' },

  YGDisplayFlex: { value: 'Display::Flex' },
  YGDisplayNone: { value: 'Display::None' },

  YGWrapNoWrap: { value: 'Wrap::NoWrap' },
  YGWrapWrap: { value: 'Wrap::Wrap' },
  YGWrapWrapReverse: { value: 'Wrap::WrapReverse' },

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

  YGNodeStyleSetBorder: {
    value: function (nodeName, edge, value) {
      var strippedValue = value.toString().replace('px', '')
      this.push(nodeName + '.set_border(' + edge + ', ' + (strippedValue) + ' as f32);')
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
      this.push(nodeName + '.set_flex_shrink(' + toRustValue(value) + ' as f32);')
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
  }
})
