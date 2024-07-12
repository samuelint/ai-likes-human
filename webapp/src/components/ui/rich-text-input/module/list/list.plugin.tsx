/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 */

import { ListItemNode, ListNode } from '@lexical/list';

import { TabIndentationPlugin } from '@lexical/react/LexicalTabIndentationPlugin';
import { ListPlugin as LexicalListPlugin } from '@lexical/react/LexicalListPlugin';


const ListNodes = [
  ListNode,
  ListItemNode,
];
export { ListNodes };

export function ListPlugin() {
  return <>
    <LexicalListPlugin />
    <TabIndentationPlugin />
  </>;
}