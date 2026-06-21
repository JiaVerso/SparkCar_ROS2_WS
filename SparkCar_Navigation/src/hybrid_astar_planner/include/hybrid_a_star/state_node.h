/*******************************************************************************
 * Software License Agreement (BSD License)
 *
 * Copyright (c) 2022 Zhang Zhimeng
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without modification,
 * are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice,
 *    this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 *  
 * |  x=0   x=1   x=2   x=3   x=4   <-- width = 5
 * |    +-----+-----+-----+-----+-----+
 * |  y=0  |  0  |  1  |  2  |  3  |  4  |
 * |    +-----+-----+-----+-----+-----+
 * |  y=1  |  5  |  6  |  7  |  8  |  9  |
 * |   +-----+-----+-----+-----+-----+
 * |  y=2  | 10  | 11  | 12  | 13  | 14  |
 * |    +-----+-----+-----+-----+-----+
 *******************************************************************************/

#ifndef HYBRID_A_STAR_STATE_NODE_H
#define HYBRID_A_STAR_STATE_NODE_H

#include "type.h"

#include <Eigen/Dense>

struct StateNode {
    EIGEN_MAKE_ALIGNED_OPERATOR_NEW

    enum NODE_STATUS {
        NOT_VISITED = 0, IN_OPENSET = 1, IN_CLOSESET = 2
    };

    enum DIRECTION {
        FORWARD = 0, BACKWARD = 1, NO = 3
    };

    StateNode() = delete;

    explicit StateNode(const Vec3i &grid_index) {
        node_status_ = NOT_VISITED;
        grid_index_ = grid_index;
        parent_node_ = nullptr;
    }

    void Reset() {
        node_status_ = NOT_VISITED;
        parent_node_ = nullptr;
    }

    NODE_STATUS node_status_;
    DIRECTION direction_{};

    Vec3d state_;
    Vec3i grid_index_;

    double g_cost_{}, f_cost_{};
    int steering_grade_{};

    StateNode *parent_node_;
    typedef StateNode *Ptr;

    VectorVec3d intermediate_states_;
};

#endif //HYBRID_A_STAR_STATE_NODE_H
