initSidebarItems({"enum":[["AddInstanceFleetError","Errors returned by AddInstanceFleet"],["AddInstanceGroupsError","Errors returned by AddInstanceGroups"],["AddJobFlowStepsError","Errors returned by AddJobFlowSteps"],["AddTagsError","Errors returned by AddTags"],["CancelStepsError","Errors returned by CancelSteps"],["CreateSecurityConfigurationError","Errors returned by CreateSecurityConfiguration"],["DeleteSecurityConfigurationError","Errors returned by DeleteSecurityConfiguration"],["DescribeClusterError","Errors returned by DescribeCluster"],["DescribeJobFlowsError","Errors returned by DescribeJobFlows"],["DescribeSecurityConfigurationError","Errors returned by DescribeSecurityConfiguration"],["DescribeStepError","Errors returned by DescribeStep"],["ListBootstrapActionsError","Errors returned by ListBootstrapActions"],["ListClustersError","Errors returned by ListClusters"],["ListInstanceFleetsError","Errors returned by ListInstanceFleets"],["ListInstanceGroupsError","Errors returned by ListInstanceGroups"],["ListInstancesError","Errors returned by ListInstances"],["ListSecurityConfigurationsError","Errors returned by ListSecurityConfigurations"],["ListStepsError","Errors returned by ListSteps"],["ModifyInstanceFleetError","Errors returned by ModifyInstanceFleet"],["ModifyInstanceGroupsError","Errors returned by ModifyInstanceGroups"],["PutAutoScalingPolicyError","Errors returned by PutAutoScalingPolicy"],["RemoveAutoScalingPolicyError","Errors returned by RemoveAutoScalingPolicy"],["RemoveTagsError","Errors returned by RemoveTags"],["RunJobFlowError","Errors returned by RunJobFlow"],["SetTerminationProtectionError","Errors returned by SetTerminationProtection"],["SetVisibleToAllUsersError","Errors returned by SetVisibleToAllUsers"],["TerminateJobFlowsError","Errors returned by TerminateJobFlows"]],"struct":[["AddInstanceFleetInput",""],["AddInstanceFleetOutput",""],["AddInstanceGroupsInput",""],["AddInstanceGroupsOutput",""],["AddJobFlowStepsInput",""],["AddJobFlowStepsOutput",""],["AddTagsInput",""],["AddTagsOutput",""],["Application",""],["AutoScalingPolicy",""],["AutoScalingPolicyDescription",""],["AutoScalingPolicyStateChangeReason",""],["AutoScalingPolicyStatus",""],["BootstrapActionConfig",""],["BootstrapActionDetail",""],["CancelStepsInfo",""],["CancelStepsInput",""],["CancelStepsOutput",""],["CloudWatchAlarmDefinition",""],["Cluster",""],["ClusterStateChangeReason",""],["ClusterStatus",""],["ClusterSummary",""],["ClusterTimeline",""],["Command",""],["Configuration"," Amazon EMR releases 4.x or later.  An optional configuration specification to be used when provisioning cluster instances, which can include configurations for applications and software bundled with Amazon EMR. A configuration consists of a classification, properties, and optional nested configurations. A classification refers to an application-specific configuration file. Properties are the settings you want to change in that file. For more information, see Configuring Applications."],["CreateSecurityConfigurationInput",""],["CreateSecurityConfigurationOutput",""],["DeleteSecurityConfigurationInput",""],["DeleteSecurityConfigurationOutput",""],["DescribeClusterInput",""],["DescribeClusterOutput",""],["DescribeJobFlowsInput",""],["DescribeJobFlowsOutput",""],["DescribeSecurityConfigurationInput",""],["DescribeSecurityConfigurationOutput",""],["DescribeStepInput",""],["DescribeStepOutput",""],["EbsBlockDevice",""],["EbsBlockDeviceConfig",""],["EbsConfiguration",""],["EbsVolume",""],["Ec2InstanceAttributes",""],["EmrClient","A client for the Amazon EMR API."],["FailureDetails",""],["HadoopJarStepConfig",""],["HadoopStepConfig",""],["Instance",""],["InstanceFleet",""],["InstanceFleetConfig",""],["InstanceFleetModifyConfig",""],["InstanceFleetProvisioningSpecifications",""],["InstanceFleetStateChangeReason",""],["InstanceFleetStatus",""],["InstanceFleetTimeline",""],["InstanceGroup",""],["InstanceGroupConfig",""],["InstanceGroupDetail",""],["InstanceGroupModifyConfig",""],["InstanceGroupStateChangeReason",""],["InstanceGroupStatus",""],["InstanceGroupTimeline",""],["InstanceResizePolicy",""],["InstanceStateChangeReason",""],["InstanceStatus",""],["InstanceTimeline",""],["InstanceTypeConfig",""],["InstanceTypeSpecification",""],["JobFlowDetail",""],["JobFlowExecutionStatusDetail",""],["JobFlowInstancesConfig",""],["JobFlowInstancesDetail",""],["KeyValue",""],["ListBootstrapActionsInput",""],["ListBootstrapActionsOutput",""],["ListClustersInput",""],["ListClustersOutput",""],["ListInstanceFleetsInput",""],["ListInstanceFleetsOutput",""],["ListInstanceGroupsInput",""],["ListInstanceGroupsOutput",""],["ListInstancesInput",""],["ListInstancesOutput",""],["ListSecurityConfigurationsInput",""],["ListSecurityConfigurationsOutput",""],["ListStepsInput",""],["ListStepsOutput",""],["MetricDimension",""],["ModifyInstanceFleetInput",""],["ModifyInstanceGroupsInput",""],["PlacementType",""],["PutAutoScalingPolicyInput",""],["PutAutoScalingPolicyOutput",""],["RemoveAutoScalingPolicyInput",""],["RemoveAutoScalingPolicyOutput",""],["RemoveTagsInput",""],["RemoveTagsOutput",""],["RunJobFlowInput",""],["RunJobFlowOutput",""],["ScalingAction",""],["ScalingConstraints",""],["ScalingRule",""],["ScalingTrigger",""],["ScriptBootstrapActionConfig",""],["SecurityConfigurationSummary",""],["SetTerminationProtectionInput",""],["SetVisibleToAllUsersInput",""],["ShrinkPolicy",""],["SimpleScalingPolicyConfiguration",""],["SpotProvisioningSpecification",""],["Step",""],["StepConfig",""],["StepDetail",""],["StepExecutionStatusDetail",""],["StepStateChangeReason",""],["StepStatus",""],["StepSummary",""],["StepTimeline",""],["SupportedProductConfig",""],["Tag",""],["TerminateJobFlowsInput",""],["VolumeSpecification",""]],"trait":[["Emr","Trait representing the capabilities of the Amazon EMR API. Amazon EMR clients implement this trait."]],"type":[["ActionOnFailure",""],["AdjustmentType",""],["ApplicationList",""],["AutoScalingPolicyState",""],["AutoScalingPolicyStateChangeReasonCode",""],["Boolean",""],["BooleanObject",""],["BootstrapActionConfigList",""],["BootstrapActionDetailList",""],["CancelStepsInfoList",""],["CancelStepsRequestStatus",""],["ClusterId",""],["ClusterState",""],["ClusterStateChangeReasonCode",""],["ClusterStateList",""],["ClusterSummaryList",""],["CommandList",""],["ComparisonOperator",""],["ConfigurationList",""],["Date",""],["EC2InstanceIdsList",""],["EC2InstanceIdsToTerminateList",""],["EbsBlockDeviceConfigList",""],["EbsBlockDeviceList",""],["EbsVolumeList",""],["ErrorCode",""],["ErrorMessage",""],["InstanceCollectionType",""],["InstanceFleetConfigList",""],["InstanceFleetId",""],["InstanceFleetList",""],["InstanceFleetState",""],["InstanceFleetStateChangeReasonCode",""],["InstanceFleetType",""],["InstanceGroupConfigList",""],["InstanceGroupDetailList",""],["InstanceGroupId",""],["InstanceGroupIdsList",""],["InstanceGroupList",""],["InstanceGroupModifyConfigList",""],["InstanceGroupState",""],["InstanceGroupStateChangeReasonCode",""],["InstanceGroupType",""],["InstanceGroupTypeList",""],["InstanceId",""],["InstanceList",""],["InstanceRoleType",""],["InstanceState",""],["InstanceStateChangeReasonCode",""],["InstanceStateList",""],["InstanceType",""],["InstanceTypeConfigList",""],["InstanceTypeSpecificationList",""],["Integer",""],["JobFlowDetailList",""],["JobFlowExecutionState",""],["JobFlowExecutionStateList",""],["KeyValueList",""],["Marker",""],["MarketType",""],["MetricDimensionList",""],["NewSupportedProductsList",""],["NonNegativeDouble",""],["ResourceId",""],["ScaleDownBehavior",""],["ScalingRuleList",""],["SecurityConfigurationList",""],["SecurityGroupsList",""],["SpotProvisioningTimeoutAction",""],["Statistic",""],["StepConfigList",""],["StepDetailList",""],["StepExecutionState",""],["StepId",""],["StepIdsList",""],["StepState",""],["StepStateChangeReasonCode",""],["StepStateList",""],["StepSummaryList",""],["StringList",""],["StringMap",""],["SupportedProductsList",""],["TagList",""],["Unit",""],["WholeNumber",""],["XmlString",""],["XmlStringList",""],["XmlStringMaxLen256",""],["XmlStringMaxLen256List",""]]});